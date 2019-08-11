use crate::instr::{Op, Event, EventKind};

/// MicroOp represents one stage of an instruction in a pipeline
/// Related to the architecture through the stage #.
/// Often abbreviated as uop(looks like μop but easy to type)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MicroOp {
  pub stage: usize,
  pub op: Op,
}

impl PartialOrd for MicroOp {
  fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
    self.op.partial_cmp(&o.op)
      .map(|cmp| cmp.then(self.stage.cmp(&o.stage)))
  }
}

impl Event for MicroOp {
  fn kind(&self) -> EventKind { self.op.kind() }
  fn same_access_address(&self, o: &Self) -> bool {
    self.op.same_access_address(&o.op)
  }
  fn same_data(&self, o: &Self) -> bool {
    self.op.same_data(&o.op)
  }
}

/// MicroOpBuckets is just a data structure
/// for partitioning the different micro ops
/// once for convenience.
#[derive(Clone, Debug)]
pub struct MicroOpBuckets {
  pub writes: Vec<MicroOp>,
  pub reads: Vec<MicroOp>,
  pub fences: Vec<MicroOp>,
  // Should only ever be one
  pub inits: Vec<MicroOp>,
}

impl MicroOpBuckets {
  fn new() -> Self {
    MicroOpBuckets{
      writes: vec!(),
      reads: vec!(),
      fences: vec!(),
      inits: vec!(),
    }
  }
}

use std::iter::FromIterator;
use std::ops::Deref;
impl<D : Deref<Target=MicroOp>> FromIterator<D> for MicroOpBuckets {
  fn from_iter<I: IntoIterator<Item=D>>(iter: I) -> Self {
    let mut out = MicroOpBuckets::new();
    for v in iter {
      let v = *v;
      if v.is_write() { out.writes.push(v) }
      else if v.is_read() { out.reads.push(v) }
      else if v.is_fence() { out.fences.push(v) }
      else if v.op.mem.is_init() { out.fences.push(v) }
    }
    out
  }
}
