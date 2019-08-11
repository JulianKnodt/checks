use crate::{
  mem::{MemOp, write, read, read_init, State, Loc},
  arch::Arch,
  micro::MicroOp,
  instr::Op,
  litmus::LitmusTest,
};

// TODO convert this into returning static slices

/// Defines a litmus test function
/// Was a pain to write.
macro_rules! litmus_test {
  ($name: ident, $( [ $($x:expr),+ ] );*) => {
    pub fn $name() -> (&'static str, LitmusTest) {
      (stringify!($name), LitmusTest::MultiThreaded(vec!($(vec!($($x),*)),*)))
    }
  };
}

litmus_test!(message_passing,
  [write(0,1), write(1,1)];
  [read(1, 1), read_init(0)]);

litmus_test!(store_buffering,
  [write(0, 1), read(1, 1)];
  [write(1, 1), read(0, 1)]);

litmus_test!(load_buffering,
  [read(0, 1), write(1, 1)];
  [read(1, 1), write(0, 1)]);

litmus_test!(single_load,
  [write(0, 1), write(0, 2), read(0, 1)]);

litmus_test!(iwp24,
  [write(0, 1), read(0, 1), read_init(1)];
  [write(1, 1), read(1, 1), read_init(0)]);

/// A list of all litmus tests that should be loaded
pub const TESTS: &[fn() -> (&'static str, LitmusTest)] =
  &[message_passing, store_buffering, load_buffering, single_load, iwp24];

pub fn convert_to_micro_ops(arch: &Arch, ops: LitmusTest) -> Vec<MicroOp> {
  ops.into_iter().flat_map(|(core, thread, pc, mem)| {
    let op = Op{core,thread,pc, mem: mem};
    (0..arch.desc.stages.len()).map(move |stage| MicroOp{stage, op})
  }).collect()
}

/// Generates all possible end states for a given Litmus Test.
/// Useful for enumerating all possible graphs to create
pub fn generate_end_states(ops: LitmusTest) -> Vec<State> {
  use std::collections::HashMap;
  let mut poss : HashMap<Loc, Vec<usize>> = HashMap::new();
  // Should create HashMap<Loc, Vec<usize>>
  // Eventually.
  ops.to_vecs().iter()
    .flat_map(|cores| cores.iter()
      .flat_map(|threads| threads.iter() .filter(|mem| mem.is_write())))
    .for_each(|w| {
      poss.entry(Loc::Addr(w.loc().unwrap()))
      .or_insert_with(|| vec!())
      .push(w.data().unwrap());
  });
  State::new_combinations(&poss)
}

#[test]
fn end_states() {
  assert_eq!(generate_end_states(message_passing().1).len(), 1);
  assert_eq!(generate_end_states(single_load().1).len(), 2);
}
