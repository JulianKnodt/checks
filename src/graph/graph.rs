/// AdjList is an adjacency list representation of a graph
/// Over a vertex type V, and an edge type E.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjList<V, E> {
  pub nodes: Vec<Option<V>>,
  pub(crate) edges: Vec<(usize, usize, E)>,
}

impl<V, E> AdjList<V, E> {
  /// New returns an empty instance of an adjacency list
  pub fn new() -> Self {
    AdjList{nodes: vec!(), edges: vec!()}
  }
  /// Adds a node to the graph and returns its index
  pub fn push_node(&mut self, v: V) -> usize {
    self.nodes.push(Some(v));
    self.nodes.len() - 1
  }
  /// #[panics]
  /// will panic either src or dst not in the graph
  ///
  /// pushes an edge between src, dst with metadata e
  pub fn push_edge(&mut self, src: usize, dst: usize, e: E) {
    assert!(self.nodes.get(src).is_some(), "Source node not in graph");
    assert!(self.nodes.get(dst).is_some(), "Destination node not in graph");
    self.edges.push((src, dst, e));
  }
  /// srcs returns the indeces of nodes who have no incoming edge
  pub fn srcs(&self) -> Vec<usize> {
    self.nodes.iter()
      .enumerate()
      .filter(|&(i, n)| n.is_some() && self.edges.iter().all(|&(_,dst,_)| dst != i))
      .map(|(i, _)| i)
      .collect()
  }
  /// sinks returns the indeces of all nodes which have no outgoing edges
  pub fn sinks(&self) -> Vec<usize> {
    self.nodes.iter()
      .enumerate()
      .filter(|&(i,n)| n.is_some() && self.edges.iter().all(|&(src,_,_)| src != i))
      .map(|(i,_)| i)
      .collect()
  }
  /// node gets the i'th node from the graph if it was not deleted.
  /// It will panic if i is greater than the number of possible nodes
  pub fn node(&self, i: usize) -> Option<&V> { self.nodes[i].as_ref() }
  /// remove_node removes the i'th node from the graph, as well as any outgoing or
  /// incoming edges, and returns the node if it existed.
  pub fn remove_node(&mut self, i: usize) -> Option<V> {
    let out = self.nodes[i].take();
    if out.is_some() {
      self.edges.retain(|&(src,dst,_)| src != i && dst != i);
    }
    out
  }
  /// has_edge returns whether there is an edge between src and dst
  pub fn has_edge(&self, src: usize, dst: usize) -> bool {
    self.edges.iter().any(|&(s, d, _)| s == src && d == dst)
  }
  /// has_edges is true if the graph has edges and false otherwise
  pub fn has_edges(&self) -> bool { !self.edges.is_empty() }
  /// neighbors returns the neighbors of a given node
  pub fn neighbors(&self, of: usize) -> Vec<usize> {
    self.edges.iter()
      .filter_map(|&(src, dst,_)| if src == of { Some(dst) } else { None })
      .collect()
  }
}

impl <V: Clone, E: Clone> AdjList<V, E> {
  /// TopoSort returns a sorted list of nodes in topological order
  /// or None if there is a cycle in the graph.
  pub fn topo_sort(&self) -> Option<Vec<usize>> {
    let mut sorted = vec!();
    let mut clone = self.clone();
    let mut srcs = clone.srcs();
    assert!(srcs.iter().all(|&i| clone.remove_node(i).is_some()));
    while let Some(v) = srcs.pop() {
      sorted.push(v);
      clone.edges.retain(|&(src,_,_)| src != v);
      clone.srcs().iter().for_each(|&src| {
          assert!(clone.remove_node(src).is_some());
          srcs.push(src);
        });
    };
    Some(sorted).filter(|_| !clone.has_edges())
  }
  /// Removes edges which return false from f
  pub fn remove_edges(&mut self, f: fn(&E) -> bool) {
    self.edges.retain(|(_, _, e)| f(e))
  }
  /// Returns whether or not the graph has a cycle
  pub fn has_cycle(&self) -> bool { self.topo_sort().is_none() }
  // returns shortest path from src to dst, including src and dst nodes.
  // if there is no path returns none
  // currently naive implementation with no prio queue
  pub fn djikstra(&self, src: usize, dst: usize) -> Option<Vec<usize>> {
    let mut nodes: Vec<_> = (0..self.nodes.len())
      .map(|i| if i == src { 0.0 } else { std::f32::INFINITY })
      .map(|dist| (dist, None, false))
      .collect();
    let mut curr_index = Some(src);
    while let Some(i) = curr_index {
      nodes[i].2 = true;
      self.neighbors(i).iter().for_each(|&n| {
        // TODO make 1.0 change depending on length?
        let alt_dist = nodes[i].0 + 1.0;
        if alt_dist < nodes[n].0 {
          nodes[n].0 = alt_dist;
          nodes[n].1 = Some(i);
        }
      });
      curr_index = nodes.iter()
        .enumerate()
        .filter(|&(_, (dist,_,visited))| !visited && dist.is_finite() && !dist.is_nan())
        .min_by(|&(_,(a,_,_)), &(_,(b,_,_))| a.partial_cmp(b).unwrap())
        .map(|(i,_)| i);
    };
    if !nodes[dst].2 { return None };
    let mut out = vec!(dst);
    let mut curr = nodes[dst].1;
    while let Some(i) = curr {
      out.push(i);
      curr = nodes[i].1;
    };
    Some(out.into_iter().rev().collect())
  }
}

/// Various tests over the cyclic nature of graphs
#[cfg(test)]
mod graph_tests {
  use super::AdjList;
  fn sample_graph() -> AdjList<(), ()> {
    let mut out = AdjList::new();
    (0..3).for_each(|_| { out.push_node(()); });
    vec!((0,1), (1,2)).iter().for_each(|&(src, dst)| {
      out.push_edge(src, dst, ());
    });
    out
  }
  fn cycle() -> AdjList<(), ()> {
    let mut out = AdjList::new();
    (0..2).for_each(|_| { out.push_node(()); });
    vec!((0,1), (1,0)).iter().for_each(|&(src, dst)| {
      out.push_edge(src, dst, ());
    });
    out
  }
  #[test]
  fn test_topo_sort() {
    let sample = sample_graph();
    let sorted = sample.topo_sort();
    assert!(sorted.is_some());
    let cyclic = cycle();
    assert!(cyclic.has_cycle());
  }
  #[test]
  fn test_djikstras() {
    assert!(sample_graph().djikstra(2, 0).is_none());
    assert_eq!(sample_graph().djikstra(0, 2), Some(vec!(0,1,2)));
  }
}

use std::iter::FromIterator;
impl<V, E> FromIterator<V> for AdjList<V, E> {
  fn from_iter<I: IntoIterator<Item=V>>(iter: I) -> Self {
    let mut out = AdjList::new();
    for v in iter {
      out.push_node(v);
    }
    out
  }
}
