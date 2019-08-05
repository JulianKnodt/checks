use crate::instr::{Op, Event};

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
  fn is_write(&self) -> bool { self.op.is_write() }
  fn is_read(&self) -> bool { self.op.is_read() }
  fn same_access_address(&self, o: &Self) -> bool {
    self.op.same_access_address(&o.op)
  }
  fn same_data(&self, o: &Self) -> bool {
    self.op.same_data(&o.op)
  }
}
