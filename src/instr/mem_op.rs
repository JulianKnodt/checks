use std::sync::atomic::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOp {
  // Init represents the initial state of the system
  Init,

  // A read might be reading uninitialized memory, represented by None
  Read(usize, Option<usize>, Ordering),
  // A write can only be writing new values
  Write(usize, usize, Ordering),

  // A reordering fence
  Fence(Ordering),
}

impl MemOp {
  pub fn loc(&self) -> Option<usize> {
    match self {
      &MemOp::Read(l,_,_) | &MemOp::Write(l, _,_) => Some(l),
      _ => None,
    }
  }
  pub fn is_write(&self) -> bool {
    if let MemOp::Write(_, _, _) = self { true } else { false }
  }
  pub fn is_read(&self) -> bool {
    if let MemOp::Read(_, _, _) = self { true } else { false }
  }
  pub fn is_init(&self) -> bool { &MemOp::Init == self }
  pub fn data(&self) -> Option<usize> {
    match self {
      &MemOp::Write(_, data, _) => Some(data),
      &MemOp::Read(_, data, _) => data,
      _ => None,
    }
  }
  pub fn same_data(&self, o: &Self) -> bool {
    match self {
      MemOp::Fence(_) => false,
      MemOp::Init => o.is_init() || (o.is_read() && o.data().is_none()),
      MemOp::Write(_, d, _) => o.data().map_or(false, |o_d| d == &o_d),

      MemOp::Read(_, d, _) => if let Some(d) = d { o.data().map_or(false, |o_d| d == &o_d) }
        else { o.is_init() || (if let MemOp::Read(_, None, _) = o { true } else { false }) }
    }
  }
}

pub fn write(loc: usize, data: usize) -> MemOp {
  MemOp::Write(loc, data, Ordering::Relaxed)
}

pub fn read(loc: usize, data: Option<usize>) -> MemOp {
  MemOp::Read(loc, data, Ordering::Relaxed)
}

pub fn fence() -> MemOp {
  MemOp::Fence(Ordering::SeqCst)
}

pub fn init() -> MemOp {
  MemOp::Init
}

