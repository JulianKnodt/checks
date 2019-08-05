use super::{Relation, Event};
use crate::graph::AdjList;

pub fn from_read<V: Event>(exec_graph: &mut AdjList<V, Relation>) {
  // CoherenceOrder edges && ReadFrom edges
  let (co, rf) : (Vec<(usize, usize, _)>, Vec<_>) = exec_graph.edges.iter()
    .filter(|(_,_,r)| r == &Relation::CoherenceOrder ||
      (if let &Relation::ReadFrom(_) = r { true } else { false }))
    .partition(|(_,_,r)| r == &Relation::CoherenceOrder);

  let reads : Vec<_> = (0..exec_graph.nodes.len()).filter(|&i|
    exec_graph.node(i).map_or(false, |n| n.is_read())
  ).collect();

  reads.into_iter().for_each(|read_idx| {
    rf.iter().filter(|(_, dst, _)| dst == &read_idx)
      .map(|rf_edge| rf_edge.0)
      .for_each(|read_from_src| {
        let mut curr = co.iter().find(|(src, _, _)| src == &read_from_src);
        while let Some((_, dst, e)) = curr {
          assert_eq!(&Relation::CoherenceOrder, e);
          exec_graph.push_edge(read_idx, *dst, Relation::FromRead);
          curr = co.iter().find(|(s,_,_)| s == dst);
        }
      });
  });
}

pub fn communications<V>(graph: &mut AdjList<V, Relation>) {
  graph.edges.iter()
    .filter(|(_,_,r)| match r {
      Relation::CoherenceOrder | Relation::ReadFrom(_) | Relation::FromRead => true,
      _ => false,
    })
    .map(|&(src, dst, _)| (src, dst))
    .collect::<Vec<_>>()
    .into_iter()
    .for_each(|(src, dst)| graph.push_edge(src, dst, Relation::Communications));
}

/*
pub fn ProgramOrderIdentLocation<V: Event>(graph: &mut AdjList<V, Relation>) {
  graph.edges.iter()
    .filter(|(src, dst, r)| r == &Relation::ProgramOrder && src.same_location(dst));
}
*/

