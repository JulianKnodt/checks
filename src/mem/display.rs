use super::MemOp;
use std::fmt::{self, Display};

impl Display for MemOp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MemOp::Init => write!(f, "Init"),

      MemOp::Write(loc, data, ord) =>
        write!(f, "Write({} @ {}, {:?})", data, loc, ord),

      MemOp::Read(loc, data, ord) => write!(f, "Read({} @ {}, {:?})", data, loc, ord),
      MemOp::ReadInit(loc, ord) => write!(f, "Read(Init @ {}, {:?})", loc, ord),

      MemOp::Fence(ord) => write!(f, "Fence({:?})", ord),
    }
  }
}
