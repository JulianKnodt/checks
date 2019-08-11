use crate::{
  arch::Arch,
  litmus::LitmusTest,
  micro::MicroOp,
  instr::Op,
};

impl LitmusTest {
  /// Returns a list of micro_ops per each memory operation
  pub fn convert_to_micro_ops(self, arch: &Arch) -> Vec<Vec<MicroOp>> {
    self.into_iter().map(|(core, thread, pc, mem)| {
      let op = Op{core,thread,pc, mem: mem};
      (0..arch.desc.stages.len()).map(move |stage| MicroOp{stage, op}).collect()
    }).collect()
  }
}
