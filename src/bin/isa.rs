use std::{
  fs::{self, File},
  process::Command,
  io::Write,
};
use checks::{
  litmus,
  instr::{execution_graph, from_read},
  graph::Graphviz,
};

fn main() {
  litmus::TESTS.iter().map(|test| test()).for_each(|(name, ops)| {
    let mut graph = execution_graph(ops);
    from_read(&mut graph);

    let file = format!("{}.dot", name);
    fs::write(
      &file,
      graph.graphviz())
      .unwrap();
    let mut png_file = File::create(format!("{}.png", name)).unwrap();
    let output = Command::new("dot")
      .arg("-Tpng")
      .arg(file)
      .output()
      .unwrap();
    png_file.write_all(&output.stdout).expect("write failed");
  });
}
