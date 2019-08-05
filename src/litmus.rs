use crate::instr::{MemOp, write, read};

/// Defines a litmus test function
/// Was a pain to write.
macro_rules! litmus_test {
  ($name: ident, $( [ $($x:expr),+ ] );*) => {
    pub fn $name() -> (&'static str, Vec<Vec<MemOp>>) {
      (stringify!($name), vec!($(vec!($($x),*)),*))
    }
  };
}

litmus_test!(message_passing,
  [write(0,1), write(1,1)];
  [read(1, Some(1)), read(0, None)]);

litmus_test!(store_buffering,
  [write(0, 1), read(1, Some(1))];
  [write(1, 1), read(0, Some(1))]);

litmus_test!(load_buffering,
  [read(0, Some(1)), write(1, 1)];
  [read(1, Some(1)), write(0, 1)]);

litmus_test!(single_load,
  [write(0, 1), write(0, 2), read(0, Some(1))]);

litmus_test!(iwp24,
  [write(0, 1), read(0, Some(1)), read(1, None)];
  [write(1, 1), read(1, Some(1)), read(0, None)]);

pub const TESTS: &[fn() -> (&'static str, Vec<Vec<MemOp>>)] =
  &[message_passing, store_buffering, load_buffering, single_load, iwp24];
