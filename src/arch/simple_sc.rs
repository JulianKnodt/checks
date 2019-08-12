use crate::{
  arch::{ArchDescription, Stage, MicroOrdering},
  instr::{Relation, Event, EventKind},
  micro::{MicroOp},
  mem::State,
};
use std::collections::HashMap;

pub const SIMPLE_SEQCST: ArchDescription = ArchDescription{
  name: "Simple-SeqCst",
  stages: &[
    Stage::new("Fetch", MicroOrdering::Queue),
    Stage::new("Execute", MicroOrdering::Queue),
    Stage::new("Writeback", MicroOrdering::Queue),
  ],
  unique_edges: Some(simple_seqcst),
};

// Returns a list of different possible edge sets that need to be added to the graph.
// Takes
fn simple_seqcst(uops: &HashMap<EventKind, Vec<(usize, MicroOp)>>, end: &State)
  -> Vec<Vec<(usize, usize, Relation)>> {
  use Relation::*;
  let writes = &uops[&EventKind::Write];
  let reads = &uops[&EventKind::Read];

  let write_serialization = writes.iter()
    .flat_map(|(i, uop)| writes.iter()
      .filter(move |(j, _)| j != i)
      .filter(move |&(_, p_uop)| p_uop.same_access_address(&uop))
      .map(move |&(j, p_uop)| if end.same_opt_data(p_uop.op.mem.loc(), p_uop.op.mem.data()) {
      vec!((*i, j, CoherenceOrder)) }
      else { vec!((*i, j, CoherenceOrder), (j, *i, CoherenceOrder)) })
    );

  /*
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
  */
  unimplemented!()
}
