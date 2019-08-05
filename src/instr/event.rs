use super::{MemOp, Op};
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
      MemOp::Init =>
        o.is_init() || (if let MemOp::Read(_, d, _) = o { d.is_none() } else { false }),

      MemOp::Fence(_) => false,

      MemOp::Read(l, d, _) =>
        (d.is_none() && o.is_init()) || o.loc().map_or(false, |o_l| &o_l == l),

      MemOp::Write(l, _, _) => o.loc().map_or(false, |o_l| &o_l == l),
    }
  }
  fn same_data(&self, o: &Self) -> bool {
    match self {
      MemOp::Init =>
        o.is_init() || (if let MemOp::Read(_,d,_) = o { d.is_none() } else { false }),
      MemOp::Fence(_) => false,

      MemOp::Read(_,None,_) => o.is_init() || (o.is_read() && o.data().is_none()),
      MemOp::Read(_,Some(d),_) => o.data().map_or(false, |o_d| &o_d == d),

      MemOp::Write(_,d,_) => o.data().map_or(false, |o_d| &o_d == d),
    }
  }
}

impl Event for Op {
  fn is_write(&self) -> bool { self.mem_op.is_write() }
  fn is_read(&self) -> bool { self.mem_op.is_read() }
  fn same_access_address(&self, o: &Self) -> bool { self.mem_op.same_access_address(&o.mem_op) }
  fn same_data(&self, o: &Self) -> bool { self.mem_op.same_data(&o.mem_op) }
}
