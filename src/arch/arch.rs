use crate::{
  micro::MicroOp,
  instr::Relation,
  mem::State,
};

/// MicroOrdering defines whether or not a stage has relationship
/// with other instructions stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroOrdering {
  Queue,
  Unordered,
}

/// Visibility defines how a stages operation modifies memory and how threads
/// will see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {

  /// read/load is exposed globally at this stage(i.e. riscv::Mem lw)
  GlobalRead,

  /// write/store is exposed locally at this stage(i.e. riscv::Mem sw)
  LocalWrite,

  /// write/store is exposed globally at this stage(i.e. riscv::Mem sw)
  GlobalWrite,

  /// Retire the instruction(ignore the rest of the stages)
  RetireWrite,
  RetireRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage{
  pub name: &'static str,
  pub ord: MicroOrdering,

  // This could technically be a set, but it will always contains very few elements
  pub vis: Option<&'static [Visibility]>,
}

impl Stage {
  pub const fn new(name: &'static str, ord: MicroOrdering, vis: Option<&'static [Visibility]>)
    -> Self {
    Stage{name, ord, vis}
  }
  pub fn has_vis(&self, vis: Visibility) -> bool {
    self.vis.as_ref().map_or(false, |vs| vs.iter().any(|v| v == &vis))
  }
}

#[derive(Clone)]
pub struct ArchDescription {
  pub name: &'static str,
  pub stages: &'static [Stage],
  pub unique_edges: Option<fn(&Vec<MicroOp>, &State) -> Vec<Vec<(MicroOp, MicroOp, Relation)>>>,
}

impl ArchDescription {
  pub const fn instance(&'static self, num_cores: usize) -> Arch {
    Arch{num_cores, desc: self}
  }
}

#[derive(Clone)]
pub struct Arch {
  pub num_cores: usize,
  pub desc: &'static ArchDescription,
}

impl Arch {
  /// Creates a graph of pipeline stages with program order edges, coherence order,
  /// and read-from edges
  pub fn stage_of(&self, vis: Visibility) -> usize {
    self.desc.stages.iter()
      .position(|s| s.vis.map_or(false, |vs| vs.iter().any(|v| v == &vis)))
      .expect(
        format!("Cannot find visibility {:?}, Missing from arch({:?})", vis, self.desc.name)
          .as_str()
      )
  }
  pub fn stages_of(&self, vis: Vec<Visibility>) -> Vec<usize> {
    vis.iter().map(|&v| self.stage_of(v)).collect()
  }
  pub fn stage(&self, i: usize) -> &'static Stage {
    &self.desc.stages[i]
  }
}
