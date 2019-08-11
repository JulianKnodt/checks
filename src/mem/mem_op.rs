use std::sync::atomic::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOp {
  // Init represents the initial state of the system
  Init,

  // Read from Init State
  ReadInit(usize, Ordering),

  // A read might be reading uninitialized memory, represented by None
  Read(usize, usize, Ordering),
  // A write can only be writing new values
  Write(usize, usize, Ordering),

  // A reordering fence
  Fence(Ordering),
}

impl MemOp {
  pub fn loc(&self) -> Option<usize> {
    match self {
      &MemOp::Read(l,_,_) | &MemOp::Write(l,_,_) | &MemOp::ReadInit(l,_) => Some(l),
      _ => None,
    }
  }
  pub fn is_write(&self) -> bool {
    if let MemOp::Write(_, _, _) = self { true } else { false }
  }
  pub fn is_read(&self) -> bool {
    match self {
      MemOp::ReadInit(_,_) | MemOp::Read(_,_,_) => true,
      _ => false,
    }
  }
  pub fn is_init(&self) -> bool { &MemOp::Init == self }
  pub fn is_read_init(&self) -> bool {
    if let MemOp::ReadInit(_,_) = self { true } else { false }
  }
  pub fn is_fence(&self) -> bool {
    if let MemOp::Fence(_) = self { true } else { false }
  }
  pub fn data(&self) -> Option<usize> {
    match self {
      &MemOp::Write(_, data, _) | &MemOp::Read(_, data, _) => Some(data),
      _ => None,
    }
  }
  pub fn same_data(&self, o: &Self) -> bool {
    self.data() == o.data()
    /*
    match self {
      MemOp::Fence(_) => false,
      MemOp::Init | MemOp::ReadInit(_,_) => o.is_init() || o.is_read_init(),
      MemOp::Write(_, d, _) | MemOp::Read(_, d, _) => Some(*d) == o.data(),
    }
    */
  }
}

pub const fn write(loc: usize, data: usize) -> MemOp {
  MemOp::Write(loc, data, Ordering::SeqCst)
}

pub const fn read(loc: usize, data: usize) -> MemOp {
  MemOp::Read(loc, data, Ordering::SeqCst)
}

pub const fn fence() -> MemOp {
  MemOp::Fence(Ordering::SeqCst)
}

pub const fn read_init(loc: usize) -> MemOp {
  MemOp::ReadInit(loc, Ordering::SeqCst)
}

pub const fn init() -> MemOp {
  MemOp::Init
}

