use super::AdjList;
use std::collections::HashSet;
use std::collections::VecDeque;

impl<V,  E> AdjList<V, E> {
  /// Returns an iterator over the nodes in a graph in depth first order.
  /// It is not guaranteed to visit every node, or have any elements.
  pub fn depth_first<'a>(&'a self, start: usize) -> DepthFirst<'a,V,E> {
    DepthFirst(vec!(start), HashSet::new(), self)
  }
  /// Returns an iterator over the nodes in a graph in breadth first order.
  pub fn breadth_first<'a>(&'a self, start: usize) -> BreadthFirst<'a,V,E> {
    let mut items = VecDeque::new();
    items.push_front(start);
    BreadthFirst(items, HashSet::new(), self)
  }
  /// Returns an iterator over the nodes in a graph, returning every layer.
  pub fn layered<'a>(&'a self, items: Vec<usize>) -> Layered<'a,V,E> {
    Layered(items, HashSet::new(), self)
  }
}

/// DepthFirst is an iterator over nodes in graph which maintains visited nodes
/// so it won't cycle forever. It will not traverse all nodes if not all nodes are reachable.
#[derive(Clone)]
pub struct DepthFirst<'a,V,E>(Vec<usize>, HashSet<usize>, &'a AdjList<V,E>);

impl<'a, V, E> Iterator for DepthFirst<'a,V,E> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    while let Some(i) = self.0.pop() {
      self.1.insert(i);
      let mut neighbors = self.2.neighbors(i).into_iter()
        .filter(|n| !self.1.contains(n))
        .rev().collect();
      self.0.append(&mut neighbors);
      if self.2.nodes[i].is_some() { return Some(i) };
    }
    return None;
  }
}

pub struct BreadthFirst<'a,V,E>(VecDeque<usize>, HashSet<usize>, &'a AdjList<V,E>);

impl<'a, V, E> Iterator for BreadthFirst<'a,V,E> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    while let Some(i) = self.0.pop_front() {
      self.1.insert(i);
      let neighbors : Vec<_> = self.2.neighbors(i).into_iter()
        .filter(|n| !self.1.contains(n))
        .collect();
      neighbors.into_iter().for_each(|n| self.0.push_back(n));
      if self.2.nodes[i].is_some() { return Some(i) };
    };
    return None;
  }
}

pub struct Layered<'a,V,E>(Vec<usize>, HashSet<usize>, &'a AdjList<V,E>);

impl<'a, V, E> Iterator for Layered<'a,V,E> {
  type Item = Vec<usize>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None };
    let out = self.0.clone();
    let items : Vec<_> = self.0.iter().map(|&i| i).collect();
    items.iter().for_each(|&i| { self.1.insert(i); });

    let next : Vec<_> = self.0.iter().flat_map(|&i|
      self.2.neighbors(i).into_iter().filter(|n| !self.1.contains(n))
    ).collect();
    self.0 = next;
    Some(out)
  }
}
