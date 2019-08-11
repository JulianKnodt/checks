use crate::{
  arch::{ArchDescription, Stage, MicroOrdering},
  instr::{Relation, Event},
  micro::{MicroOp},
  mem::State,
};

pub const SIMPLE_SEQCST: ArchDescription = ArchDescription{
  name: "Simple-SeqCst",
  stages: &[
    Stage::new("Fetch", MicroOrdering::Queue, None),
    Stage::new("Execute", MicroOrdering::Queue, None),
    Stage::new("Writeback", MicroOrdering::Queue, None),
  ],
  unique_edges: Some(simple_seqcst),
};

// Returns a list of different scenarios
fn simple_seqcst(uops: &Vec<MicroOp>, end: &State) -> Vec<Vec<(MicroOp, MicroOp, Relation)>> {
  use Relation::*;

  let _write_serialization = uops.iter().enumerate().filter(|(_,uop)| uop.is_write())
    .flat_map(|(i, &uop)| uops.iter().take(i)
    .filter(move |&p_uop| p_uop != &uop && p_uop.is_write() && p_uop.same_access_address(&uop))
    .map(move |&p_uop| if end.same_opt_data(p_uop.op.mem.loc(), p_uop.op.mem.data()) {
      vec!((uop, p_uop, CoherenceOrder)) }
      else { vec!((uop, p_uop, CoherenceOrder), (p_uop, uop, CoherenceOrder)) })
    );

  let _enforce_write_ordering = uops.iter()
    .filter(|uop| uop.is_write() && uop.stage == 2)
    .flat_map(|&uop| uops.iter()
      .filter(|p_uop| p_uop.stage == 1)
      .filter(move |p_uop| p_uop.op.same_core(&uop.op) && p_uop.op.imm_precedes(&uop.op))
      .map(move |p_uop| vec!((uop, p_uop, Relation::Special("EnforceWriteOrdering")))));

  let _before_all_writes = uops.iter()
    .filter(|uop| uop.op.mem.is_read_init() && uop.stage == 1)
    .flat_map(|uop| uops.iter()
      .filter(|p_uop| p_uop.stage == 2)
      .filter(move |p_uop| p_uop.is_write() && uop.same_access_address(&p_uop))
      .map(move |p_uop| vec!((uop, p_uop, FromRead))));
  unimplemented!()
}
