use super::{Op, MemOp};
use crate::graph::{AdjList, Graphviz};
use crate::instr::Relation;
use std::fmt::{self, Display};

impl Display for Op {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Core({})|Thread({})|PC({})|{{{}}}",
      self.core, self.thread, self.pc, self.mem_op)
  }
}

impl Display for MemOp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MemOp::Init => write!(f, "Init"),

      MemOp::Write(loc, data, ord) =>
        write!(f, "Write|Loc({})|Data({})|Ord({:?})", loc, data, ord),

      MemOp::Read(loc, data, ord) =>
        write!(f, "Read|Loc({})|Data({})|Ord({:?})",
          loc, data.map_or(String::from("Init"), |d| format!("{}", d)), ord),

      MemOp::Fence(ord) => write!(f, "Fence({:?})", ord),
    }
  }
}


impl Graphviz for AdjList<Op, Relation> {
  fn graphviz(&self) -> String {
    use std::collections::HashMap;
    let mut pc_map = HashMap::new();
    self.nodes.iter().enumerate()
      .filter_map(|(i, op)| op.as_ref().map(|op| (i, op)))
      .filter(|(_, op)| !op.mem_op.is_init())
      .for_each(|(i, op)|
        pc_map.entry(op.pc).or_insert_with(|| vec!()).push(i)
      );
    let ranks = pc_map.values().fold(String::from("{ rank = same; 0 }"), |acc, v| {
      let nested = v.iter().fold(String::from(""), |acc, node| format!("{}{}; ", acc, node));
      format!("{}{{ rank = same; {} }}\n", acc, nested)
    });
    let nodes = self.nodes.iter()
      .enumerate()
      .filter_map(|(i, n)| n.as_ref().map(|n| (i, n)))
      .fold(String::from(""), |acc, (i, n)| format!("{}  {} [label=\"{}\"]\n", acc, i, n));
    let edges = self.edges.iter()
      .fold(String::from(""), |acc, (src, dst, e)|
        format!("{}  {} -> {} [label=\"{:?}\"]\n", acc, src, dst, e));
     format!("digraph {{\n\tnode [shape = record];\n\trankdir=TB\n{}{}{}}}",
      ranks, nodes, edges)
  }
}
