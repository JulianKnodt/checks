use crate::{
  graph::AdjList,
  instr::{Op, Relation, Event, EventBuckets, Locality},
  mem::MemOp,
  arch::{Arch, Visibility, MicroOrdering},
  micro::{MicroOp},
  litmus::LitmusTest,
};
impl Arch {
  pub fn create_micro_graph(&self, test: LitmusTest) -> AdjList<MicroOp, Relation> {
    let mut out = AdjList::new();

    let uops = test.convert_to_micro_ops(self);

    uops.iter().fold(None, |prev : Option<Vec<usize>>, next| {
      let positions = next.iter().map(|&uop| {
          let index = out.push_node(uop);
          let stage = self.stage(uop.stage);
          if uop.stage != 0 { out.push_edge(index -1, index, Relation::ProgramOrder) }
          index
        }).collect();
      Some(positions)
    });

    out
  }
}









