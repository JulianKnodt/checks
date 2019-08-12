use crate::{
  arch::{Stage, MicroOrdering, ArchDescription},
};

/// RISC-V implementation
pub const RISCV: ArchDescription = ArchDescription{
  name: "RISC-V",
  stages: &[
    Stage::new("Instruction Fetch", MicroOrdering::Queue),
    Stage::new("Instruction Decode", MicroOrdering::Queue),
    Stage::new("EXecute", MicroOrdering::Queue),
    Stage::new("MEMory", MicroOrdering::Queue),
    Stage::new("WriteBack", MicroOrdering::Queue),
  ],
  unique_edges: None,
};

/// RISC-V implementation with store buffer
pub const RISCV_SBUF: ArchDescription = ArchDescription{
  name: "RISC-V",
  stages: &[
    Stage::new("Instruction Fetch", MicroOrdering::Queue),
    Stage::new("Instruction Decode", MicroOrdering::Queue),
    Stage::new("EXecute", MicroOrdering::Queue),
    Stage::new("MEMory", MicroOrdering::Queue),
    Stage::new("WriteBack", MicroOrdering::Queue),
    Stage::new("Store Buffer", MicroOrdering::Queue),
    Stage::new("Memory Hierarchy", MicroOrdering::Unordered),
    Stage::new("Completed", MicroOrdering::Unordered),
  ],
  unique_edges: None,
};

