use crate::{
  arch::Arch,
  litmus::LitmusTest,
  micro::MicroOp,
  instr::Op,
  mem::{State, Loc},
};
use std::collections::HashMap;

impl LitmusTest {
  /// Returns micro ops indexed by (core, thread, pc, stage)
  pub fn to_uops(self, arch: &Arch) ->
    HashMap<(usize, usize, usize, usize), MicroOp> {
    self.into_iter().flat_map(|(core, thread, pc, mem)| {
      let op = Op{core,thread,pc, mem: mem};
      (0..arch.desc.stages.len())
        .map(move |stage| ((core, thread, pc, stage), MicroOp{stage, op}))
    }).collect()
  }
  /// Generates all possible end states for a given Litmus Test.
  /// Useful for enumerating all possible graphs to create
  pub fn end_states(self) -> Vec<State> {
    let mut poss : HashMap<Loc, Vec<usize>> = HashMap::new();
    // Should create HashMap<Loc, Vec<usize>>
    // Eventually.
    self.to_vecs().iter()
      .flat_map(|cores| cores.iter()
        .flat_map(|threads| threads.iter() .filter(|mem| mem.is_write())))
      .for_each(|w| {
        poss.entry(Loc::Addr(w.loc().unwrap()))
        .or_insert_with(|| vec!())
        .push(w.data().unwrap());
    });
    State::new_combinations(&poss)
  }

}
#[test]
fn end_states() {
  use litmus::{message_passing, single_load};
  assert_eq!(message_passing().1.end_states().len(), 1);
  assert_eq!(single_load().1.end_states().len(), 2);
}
