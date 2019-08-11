use super::{Relation, Op, Event, Locality};
use crate::{
  graph::AdjList,
  mem::MemOp,
  litmus::LitmusTest,
};

/// Creates a graph at the instruction level with
/// Program Order, ReadFrom, and Coherence Order edges
pub fn execution_graph(litmus: LitmusTest) -> AdjList<Op, Relation> {
  let mut out = AdjList::new();
  let init_state = out.push_node(Op::init_marker());
  litmus.into_iter().for_each(|(core, thread, pc, mem)| {
    let op = Op{core, thread, pc, mem};
    let index = out.push_node(op);
    out.push_edge(if pc == 0 { init_state } else { index - 1 }, index, Relation::ProgramOrder);
  });

  // Adding CoherenceOrder edges
  // This implementation might not be correct
  let writes : Vec<_> = out.nodes.iter().enumerate()
    .filter_map(|(i, n)| n.map(|op| (i, op)))
    .filter_map(|v| Some(v).filter(|v| v.1.is_write()))
    .collect();
  (0..writes.len()).for_each(|i| {
    let (node_i, curr) = writes[i];
    assert!(curr.is_write());
    let no_prev_writes = 0 == (0..i).filter_map(|j| {
      let (node_j, prev) = writes[j];
      assert!(prev.is_write());
      Some(node_j).filter(|_| prev.mem.loc() == curr.mem.loc())
    })
    .inspect(|&node_j| out.push_edge(node_j, node_i, Relation::CoherenceOrder))
    .count();
    if no_prev_writes {
      out.push_edge(init_state, node_i, Relation::CoherenceOrder);
    }
  });
  let reads : Vec<_> = out.nodes.iter().enumerate()
    .filter_map(|(i, n)| n.map(|op| (i, op)))
    .filter_map(|v| if v.1.is_read() { Some(v) } else { None })
    .collect();

  // Add ReadFrom edges
  reads.iter()
    .for_each(|&(read_index, read)| match read.mem.data() {
      None => out.push_edge(init_state, read_index, Relation::ReadFrom(Locality::Global)),
      Some(data) => {
        let matching_writes : Vec<_> = writes.iter()
          .filter(|(_, w)| w.same_access_address(&read) && w.mem.data().unwrap() == data)
          .collect();
        if matching_writes.is_empty() {
          panic!("Defining graph for program is impossible, \
          no matching write for read");
        }
        matching_writes.iter().for_each(|(write_index, _)|
            out.push_edge(*write_index, read_index, Relation::ReadFrom(Locality::Global)));
      },
    });
  out
}

