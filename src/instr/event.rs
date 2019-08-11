use super::{Op};
use crate::mem::MemOp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind { Read, Write, Fence, Init }
pub trait Event {
  fn kind(&self) -> EventKind;
  fn is_read(&self) -> bool { EventKind::Read == self.kind() }
  fn is_write(&self) -> bool { EventKind::Write == self.kind() }
  fn is_fence(&self) -> bool { EventKind::Fence == self.kind() }
  fn is_init(&self) -> bool { EventKind::Init == self.kind() }
  fn same_access_address(&self, o: &Self) -> bool;
  fn same_data(&self, o: &Self) -> bool;
}

impl Event for MemOp {
  fn kind(&self) -> EventKind {
    use MemOp::*;
    match self {
      ReadInit(_,_) | Read(_,_,_) => EventKind::Read,
      Write(_,_,_) => EventKind::Write,
      Fence(_) => EventKind::Fence,
      Init => EventKind::Init,
    }
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
  fn kind(&self) -> EventKind { self.mem.kind() }
  fn same_access_address(&self, o: &Self) -> bool { self.mem.same_access_address(&o.mem) }
  fn same_data(&self, o: &Self) -> bool { self.mem.same_data(&o.mem) }
}

/// EventBuckets is just a data structure
/// for partitioning the different micro ops
/// once for convenience.
#[derive(Clone, Debug)]
pub struct EventBuckets<E> {
  pub writes: Vec<(usize, E)>,
  pub reads: Vec<(usize, E)>,
  pub fences: Vec<(usize, E)>,
  pub inits: Vec<(usize, E)>,
}

impl<E> EventBuckets<E> {
  fn new() -> Self {
    EventBuckets{
      writes: vec!(),
      reads: vec!(),
      fences: vec!(),
      inits: vec!(),
    }
  }
}

use std::iter::FromIterator;
use std::ops::Deref;
impl<E: Event + Clone, D : Deref<Target=E>> FromIterator<D> for EventBuckets<E> {
  fn from_iter<I: IntoIterator<Item=D>>(iter: I) -> Self {
    let mut out = EventBuckets::new();
    for (i, v) in iter.into_iter().enumerate() {
      let bucket = match v.kind() {
        EventKind::Write => &mut out.writes,
        EventKind::Read => &mut out.reads,
        EventKind::Fence => &mut out.fences,
        EventKind::Init => &mut out.inits,
      };
      bucket.push((i, v.clone()));
    }
    out
  }
}
