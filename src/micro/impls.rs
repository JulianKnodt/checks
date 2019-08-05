use super::{Visibility::*, Arch, ArchDescription, MicroOrdering, Stage};

/// RISC-V implementation
pub const RISCV: ArchDescription = ArchDescription{
  name: "RISC-V",
  non_local_mappings: &[],
  stages: &[
    Stage::new("Instruction Fetch", MicroOrdering::Queue, None),
    Stage::new("Instruction Decode", MicroOrdering::Queue, None),
    Stage::new("EXecute", MicroOrdering::Queue, None),
    Stage::new("MEMory", MicroOrdering::Queue,
      Some(&[GlobalRead, LocalWrite])),
    Stage::new("WriteBack", MicroOrdering::Queue,
      Some(&[GlobalWrite, RetireRead, RetireWrite])),
  ],
};

pub const RISCV_SBUF: ArchDescription = ArchDescription{
  name: "RISC-V",
  stages: &[
    Stage::new("Instruction Fetch", MicroOrdering::Queue, None),
    Stage::new("Instruction Decode", MicroOrdering::Queue, None),
    Stage::new("EXecute", MicroOrdering::Queue, None),
    Stage::new("MEMory", MicroOrdering::Queue,
      Some(&[GlobalRead, LocalWrite])),
    Stage::new("WriteBack", MicroOrdering::Queue,
      Some(&[RetireRead])),
    Stage::new("Store Buffer", MicroOrdering::Queue, None),
    Stage::new("Memory Hierarchy", MicroOrdering::Unordered,
      Some(&[GlobalWrite])),
    Stage::new("Completed", MicroOrdering::Unordered,
      Some(&[RetireWrite])),
  ],
  non_local_mappings: &[],
};

pub const ARCH_DESCS: &[ArchDescription] =
  &[RISCV_SBUF];

pub fn archs(cores: usize) -> Vec<Arch> {
  ARCH_DESCS.iter().map(|desc| desc.instance(cores)).collect()
}
