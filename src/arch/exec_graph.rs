use crate::{
  graph::AdjList,
  instr::{Relation, Event},
  arch::{Arch, MicroOrdering},
  micro::{MicroOp},
  litmus::LitmusTest,
  mem::State,
};
use std::collections::HashMap;

impl Arch {
  pub fn create_micro_graph(&self, test: LitmusTest) -> AdjList<MicroOp, Relation> {
    let mut out = AdjList::new();

    let uops = test.to_uops(self);
    let end_states = test.end_states();

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

    let choices : Vec<(&State, Vec<Vec<(usize, usize, Relation)>>)> = self.desc.unique_edges
      .as_ref()
      .map(|mapping_fn| {
      let mut input = HashMap::new();
      inserted.iter()
        .for_each(|(_, v)|
          input.entry(v.1.kind()).or_insert_with(|| vec!()).push(*v)
        );
      unimplemented!();
      end_states.iter().map(move |state| (state, mapping_fn(&input, state))).collect()
    }).unwrap_or_else(|| vec!());

    out
  }
}









