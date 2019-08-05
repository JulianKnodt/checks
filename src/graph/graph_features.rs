use super::AdjList;
use std::cmp::Ordering;

impl <V: PartialEq, E: PartialEq>AdjList<V, E> {
  pub fn is_subgraph_of(&self, o: &Self) -> bool {
    o.nodes.iter().enumerate().all(|(i, v)|
      self.nodes.get(i).map_or(false, |v2| v.eq(v2))
    ) &&
    o.edges.iter().all(|&(src, dst, _)| self.has_edge(src, dst))
  }
}

impl<V, E> AdjList<V, E> {
  /// returns the in-degree of a graph
  pub fn in_degree(&self, i: usize) -> usize {
    self.edges.iter()
      .filter(|(_,dst,_)| *dst == i)
      .count()
  }
  /// returns the out degree of a node. If the node is not in the graph
  pub fn out_degree(&self, i: usize) -> usize {
    self.edges.iter()
      .filter(|(src,_,_)| *src == i)
      .count()
  }
  /// gets any non-empty node in the graph
  /// or None if there are none
  pub fn any_node(&self) -> Option<usize> {
    (0..self.nodes.len()).find(|&n| self.nodes[n].is_some())
  }

  pub fn minimum_in_degrees(&self) -> Vec<usize> {
    (0..self.nodes.len()).fold((std::usize::MAX, vec!()), |(min, mut nodes), n| {
      let in_degree = self.in_degree(n);
      match in_degree.cmp(&min) {
        Ordering::Greater => (min, nodes),
        Ordering::Equal => {
          nodes.push(n);
          (min, nodes)
        },
        Ordering::Less => (in_degree, vec!(n)),
      }
    }).1
  }
}
