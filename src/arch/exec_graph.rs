use crate::{
  graph::AdjList,
  instr::{Op, Relation, Event, EventBuckets, Locality},
  mem::MemOp,
  arch::{Arch, Visibility, MicroOrdering},
  micro::{MicroOp},
  litmus::LitmusTest,
};
use std::collections::HashMap;

impl Arch {
  pub fn create_micro_graph(&self, test: LitmusTest) -> AdjList<MicroOp, Relation> {
    let mut out = AdjList::new();

    let uops = test.to_uops(self);

    // insert into adj list in arbitrary order
    let inserted : HashMap<(usize, usize, usize, usize),_> = uops.iter()
      .map(|(&k, &uop)| (k, (out.push_node(uop), uop)))
      .collect();

    inserted.iter().for_each(|(&(core, thread, pc, stage), &(i, _))| {
      if stage != 0 {
        out.push_edge(inserted[&(core, thread, pc, stage-1)].0, i, Relation::ProgramOrder)
      }
      let arch_stage = self.stage(stage);
      if pc != 0 && arch_stage.ord == MicroOrdering::Queue {
        out.push_edge(inserted[&(core, thread, pc-1, stage)].0, i, Relation::StageOrder)
      }
    });

    out
  }
}









