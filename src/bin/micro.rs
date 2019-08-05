use std::{
  fs::{self, File},
  process::Command,
  io::Write,
};
use checks::{
  litmus,
  micro::impls::archs,
  graph::Graphviz,
};

fn main() {
  archs(5).iter().for_each(|arch| {
    litmus::TESTS.iter().map(|test| test()).for_each(|(name, test)| {
      let file = format!("{}.dot", name);
      fs::write(&file, arch.create_micro_graph(&test).graphviz()).unwrap();
      let mut png_file = File::create(format!("{}.png", name)).unwrap();
      let output = Command::new("dot").arg("-Tpng").arg(file).output().unwrap();
      png_file.write_all(&output.stdout).expect("write failed");
    });
  });
}
