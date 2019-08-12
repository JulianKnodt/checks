use crate::{
  micro::MicroOp,
  instr::{Relation, EventKind},
  mem::State,
};
use std::collections::HashMap;

/// MicroOrdering defines whether or not a stage has relationship
/// with other instructions stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroOrdering {
  Queue,
  Unordered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage{
  pub name: &'static str,
  pub ord: MicroOrdering,
}

impl Stage {
  pub const fn new(name: &'static str, ord: MicroOrdering) -> Self {
    Stage{name, ord, }
  }
}

type MappingFunction = fn(&HashMap<EventKind, Vec<(usize, MicroOp)>>, end: &State) ->
  Vec<Vec<(usize, usize, Relation)>>;

/// A static description of a micro architectural pipeline
/// that also contains a way to define additional edges.
#[derive(Clone)]
pub struct ArchDescription {
  pub name: &'static str,
  pub stages: &'static [Stage],

  // In theory all of them should have a mapping function, but easily disable with option
  pub unique_edges: Option<MappingFunction>,
}

impl ArchDescription {
  pub const fn instance(&'static self, num_cores: usize) -> Arch {
    Arch{num_cores, desc: self}
  }
}

/// An instance of an architecture description with a specified number of cores
#[derive(Clone)]
pub struct Arch {
  pub num_cores: usize,
  pub desc: &'static ArchDescription,
}

impl Arch {
  /// Stage returns the i'th stage of the architecture pipeline.
  pub fn stage(&self, i: usize) -> &'static Stage {
    &self.desc.stages[i]
  }
}
