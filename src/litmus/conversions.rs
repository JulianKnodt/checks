use crate::{
  arch::Arch,
  litmus::LitmusTest,
  micro::MicroOp,
  instr::Op,
};
use std::collections::HashMap;

impl LitmusTest {
  /// Returns micro ops indexed by (core, thread, pc, stage)
  pub fn to_uops(self, arch: &Arch) ->
    HashMap<(usize, usize, usize, usize), MicroOp> {
    self.into_iter().flat_map(|(core, thread, pc, mem)| {
      let op = Op{core,thread,pc, mem: mem};
      (0..arch.desc.stages.len())
        .map(move |stage| ((core, thread, pc, stage), MicroOp{stage, op}))
    }).collect()
  }
}
