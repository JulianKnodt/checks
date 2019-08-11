use crate::{
  graph::AdjList,
  micro::MicroOp,
  instr::Relation,
};

fn micro_op_graph(uops: &Vec<MicroOp>) -> AdjList<MicroOp, Relation> {
  let out : AdjList<_,_> = uops.iter().cloned().collect();
  out
}
