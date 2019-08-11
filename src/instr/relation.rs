#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locality {
  Global,
  Local,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relation {
  // Execution
  ProgramOrder,
  StageOrder,
  ReadFrom(Locality),

  /// Defines Ordering of writes to same location
  /// Alternatively could be called WriteSerialization
  CoherenceOrder,

  /// Special Edges should be used for MicroArchitectural Non-Local-Edges
  /// which maintain special rules
  Special(&'static str),


  // Architecture-Dependent
  PreservedProgramOrder,
  Fence,
  PropogationOrder,

  // Derived
  FromRead,
  HappensBefore,
  Communications,

  // Debug
  Debug,
}

impl Relation {
  fn just_caps(s: String) -> String {
    assert!(s.is_ascii());
    s.chars()
      .filter(|c| c.is_ascii_uppercase())
      .map(|c| c.to_ascii_lowercase())
      .collect::<String>()
  }
  pub fn label(&self) -> String {
    Relation::just_caps(format!("{:?}", self))
  }
  pub fn color(&self) -> &'static str {
    use {Relation::*, Locality::*};
    match self {
      ProgramOrder => "#000000",
      StageOrder => "#27556C",
      ReadFrom(Global) => "#48C9B0",
      ReadFrom(Local) => "#A569BD",
      _ => "#AAAAAA",
    }
  }
  pub fn is_constraint(&self) -> bool {
    use Relation::*;
    match self {
      ProgramOrder | StageOrder => true,
      _ => false,
    }
  }
}
