use std::{
  fs::{self, File},
  process::Command,
  io::Write,
};
use checks::{
  litmus,
  graph::Graphviz,
  arch::archs,
};

fn format_arch_name(n: &str) -> String {
  n.chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect()
}

fn main() {
  archs(5).iter().for_each(|arch| {
    let arch_name = format_arch_name(arch.desc.name);
    litmus::TESTS.iter().map(|test| test()).for_each(|(name, test)| {
      let file = format!("{}.dot", name);
      fs::write(&file, arch.create_micro_graph(test).graphviz()).unwrap();
      let mut png_file = File::create(format!("{}.{}.png", name, arch_name)).unwrap();
      let output = Command::new("dot").arg("-Tpng").arg(file).output().unwrap();
      png_file.write_all(&output.stdout).expect("write failed");
    });
  });
}
