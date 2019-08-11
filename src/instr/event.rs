use super::{Op};
use crate::mem::MemOp;
pub trait Event {
  fn is_read(&self) -> bool;
  fn is_write(&self) -> bool;
  fn same_access_address(&self, o: &Self) -> bool;
  fn same_data(&self, o: &Self) -> bool;
}

impl Event for MemOp {
  fn is_write(&self) -> bool {
    if let MemOp::Write(_, _, _) = self { true } else { false }
  }
  fn is_read(&self) -> bool {
    if let MemOp::Read(_, _, _) = self { true } else { false }
  }
  fn same_access_address(&self, o: &Self) -> bool {
    match self {
      MemOp::Init => o.is_init() || o.is_read_init(),
      MemOp::ReadInit(l,_) => o.is_init() || (Some(*l) == o.loc()),
      MemOp::Fence(_) => false,
      MemOp::Read(l,_,_) | MemOp::Write(l, _, _) => Some(*l) == o.loc(),
    }
  }
  fn same_data(&self, o: &Self) -> bool {
    match self {
      MemOp::Init | MemOp::ReadInit(_,_) => o.is_init() || o.is_read_init(),
      MemOp::Fence(_) => false,
      MemOp::Read(_,d,_) | MemOp::Write(_,d,_) => Some(*d) == o.data(),
    }
  }
}

impl Event for Op {
  fn is_write(&self) -> bool { self.mem.is_write() }
  fn is_read(&self) -> bool { self.mem.is_read() }
  fn same_access_address(&self, o: &Self) -> bool { self.mem.same_access_address(&o.mem) }
  fn same_data(&self, o: &Self) -> bool { self.mem.same_data(&o.mem) }
}
