use super::MicroOp;
use crate::graph::{AdjList, Graphviz};
use crate::instr::Relation;
use std::fmt::{self, Display};

impl Display for MicroOp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.stage)//, self.op)
  }
}

impl Graphviz for AdjList<MicroOp, Relation> {
  fn graphviz(&self) -> String {
    use std::collections::HashMap;
    let mut stage_map = HashMap::new();
    self.nodes.iter().enumerate()
      .filter_map(|(i, uop)| uop.as_ref().map(|uop| (i, uop)))
      .filter(|(_, uop)| !uop.op.mem.is_init())
      .for_each(|(i, uop)|
        stage_map.entry(uop.stage).or_insert_with(|| vec!()).push(i)
      );
    let ranks = stage_map.values().fold(String::from("{ rank = same; 0 }\n"), |acc, v| {
      let nested = v.iter().fold(String::from(""), |acc, node| format!("{}{}; ", acc, node));
      format!("{}{{ rank = same; {}}}\n", acc, nested)
    });
    let nodes = self.nodes.iter()
      .enumerate()
      .filter_map(|(i, n)| n.as_ref().map(|n| (i, n)))
      .fold(String::from(""), |acc, (i, n)| format!("{}  {} [label=\"{}\"]\n", acc, i, n));
    let edges = self.edges.iter()
      .fold(String::from(""), |acc, (src, dst, e)|
        format!("{}  {} -> {} [constraint = {} color = \"{}\" label=\"{}\"]\n",
          acc, src, dst, e.is_constraint(), e.color(), e.label()));
     format!("digraph {{\n\tnode [shape = circle];\n\trankdir=TB\n\toverlap=false\n{}{}{}}}",
       ranks, nodes, edges)
  }
}
