use crate::mem::{MemOp, init};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op {
  pub core: usize,
  pub thread: usize,
  pub pc: usize,
  pub mem: MemOp,
}

impl Op {
  /// A marker for the beginning of the program
  /// should be referenced by reads from uninitialized memory
  pub fn init_marker() -> Self {
    Op{
      core: 0,
      thread: 0,
      pc: 0,
      mem: init(),
    }
  }
  pub fn same_core(&self, o: &Self) -> bool { self.core == o.core }
  pub fn same_thread(&self, o: &Self) -> bool { self.thread == o.thread }
  pub fn imm_precedes(&self, o: &Self) -> bool { self.pc + 1 == o.pc }
  pub fn precedes(&self, o: &Self) -> bool { self.pc < o.pc }

  pub fn new(core: usize, thread: usize, pc: usize, mem: MemOp) -> Self {
    Op{core, thread, pc, mem}
  }
}

impl PartialOrd for Op {
  fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
    if !(self.same_core(o) && self.same_thread(o)) { None }
    else { Some(self.pc.cmp(&o.pc)) }
  }
}
