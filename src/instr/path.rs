use crate::{
  instr::Op,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
  /// List of stages this instruction went through
  pub stages: Vec<usize>,

  pub micro_op: MicroOp,
}
