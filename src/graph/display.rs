pub trait Graphviz {
  fn graphviz(&self) -> String;
}

/*
impl<V: Display, E: Debug> Graphviz for AdjList<V, E> {
  fn graphviz(&self) -> String {
    let ranks = self.layered(vec!(0)).fold(String::from(""), |acc, nexts| {
      format!("{}\n{{ rank = same; {}}}",
        acc,
        nexts.into_iter().map(|i| format!("{}; ", i)).collect::<Vec<_>>().join(" "))
    });
    let nodes = self.nodes.iter()
      .enumerate()
      .filter_map(|(i, n)| n.as_ref().map(|v| (i, v)))
      .fold(String::from(""), |acc, (i, n)| format!("{}\n\t{} [label=\"{}\"]", acc, i, n));
    let edges = self.edges.iter()
      .fold(String::from(""), |acc, (src, dst, content)|
        format!("{}\n\t{} -> {} [label=\"{:?}\"]", acc, src, dst, content));
    format!("digraph {{{ranks}{nodes}{edges}\n}}", ranks=ranks, nodes=nodes, edges=edges)
  }
}
*/
