use crate::{
  arch::{Stage, MicroOrdering, Visibility, ArchDescription},
};
use Visibility::*;

/// RISC-V implementation
pub const RISCV: ArchDescription = ArchDescription{
  name: "RISC-V",
  stages: &[
    Stage::new("Instruction Fetch", MicroOrdering::Queue, None),
    Stage::new("Instruction Decode", MicroOrdering::Queue, None),
    Stage::new("EXecute", MicroOrdering::Queue, None),
    Stage::new("MEMory", MicroOrdering::Queue,
      Some(&[GlobalRead, LocalWrite])),
    Stage::new("WriteBack", MicroOrdering::Queue,
      Some(&[GlobalWrite, RetireRead, RetireWrite])),
  ],
  unique_edges: None,
};

/// RISC-V implementation with store buffer
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
  unique_edges: None,
};

