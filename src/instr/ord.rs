use std::sync::atomic::Ordering;

pub trait Ordered {
  fn ord(&self) -> Ordering;
}

fn permutations_of<O: Ordered>(v: &[O]) -> Vec<Vec<usize>> {
  fn helper<O: Ordered>(v: &[O], offset: usize) -> Vec<Vec<usize>> {
  }
  helper(v, 0)
}

pub fn orderings_of<O: Ordered>(v: &[O]) -> Vec<Vec<Vec<usize>>> {
  fn helper<O: Ordered>(v: &[O], offset: usize) -> Vec<Vec<usize>> {
  }
  helper(v, 0)
}
