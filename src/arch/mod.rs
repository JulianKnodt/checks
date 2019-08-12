mod arch;
pub use arch::*;

pub mod riscv;

pub mod simple_sc;

mod exec_graph;
pub use exec_graph::*;

pub const ARCH_DESCRIPTIONS: &[arch::ArchDescription] =
  &[riscv::RISCV, simple_sc::SIMPLE_SEQCST];

pub fn archs(core: usize) -> Vec<arch::Arch> {
  ARCH_DESCRIPTIONS.iter().map(|desc| desc.instance(core)).collect()
}
