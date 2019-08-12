use crate::{
  mem::MemOp,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitmusTest {
  SingleThreaded(&'static[MemOp]),
  MultiThreaded(&'static[&'static[MemOp]]),
  MultiCore(&'static[&'static[&'static[MemOp]]]),
}
use LitmusTest::*;

impl IntoIterator for LitmusTest {
  type Item = (usize, usize, usize, MemOp);
  type IntoIter = ::std::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    let v : Vec<Self::Item> = match self {
      SingleThreaded(ops) =>
        ops.into_iter().enumerate().map(|(pc, &mem)| (0, 0, pc, mem)).collect(),

      MultiThreaded(threads) => threads.into_iter().enumerate()
        .flat_map(|(thread, ops)| ops.into_iter().enumerate()
          .map(move |(pc, &mem)| (0, thread, pc, mem))).collect(),

      MultiCore(cores) => cores.into_iter().enumerate()
        .flat_map(move |(core, threads)| threads.into_iter().enumerate()
          .flat_map(move |(thread, ops)| ops.into_iter().enumerate()
            .map(move |(pc, &mem)| (core, thread, pc, mem)))).collect(),

    };
    v.into_iter()
  }
}

impl LitmusTest {
  pub fn to_vecs(self) -> Vec<Vec<Vec<MemOp>>> {
    match self {
      SingleThreaded(ops) => vec!(vec!(ops.iter().copied().collect())),
      MultiThreaded(threads) => vec!(threads.iter().map(|ops| {
        ops.iter().copied().collect()
      }).collect()),
      MultiCore(cores) => cores.iter().map(|threads|
          threads.iter().map(|ops| ops.iter().copied().collect()
        ).collect()
      ).collect()
    }
  }
}
