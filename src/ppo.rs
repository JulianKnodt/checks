use crate::{
  bell::unique_points,
  mem::{MemOp, read, write},
};

// MemType is a local enum that maps to writes/reads of 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MemType {R, W}
impl MemType {
  pub fn to_op(&self, loc: usize) -> MemOp {
    match self {
      MemType::R => read(loc, 0),
      MemType::W => write(loc, 0),
    }
  }
}

fn all_location_combos(v: &'static [MemType]) -> Vec<Vec<MemOp>> {
  unique_points(v.len()).iter().map(|mapping| {
    v.clone().into_iter()
      .enumerate()
      .map(|(i, memt)| memt.to_op(mapping[i]))
      .collect()
  }).collect()
}

fn check_ppo_any_addr(mts: &'static [MemType]) {

}



// GraphsToVerifyPreservedProgramOrderAnyAddress
// GraphsToVerifyPreservedProgramOrderSameAddress
// [R, R], [R, W], [W, W], [R,R]

use MemType::*;
const TO_VERIFY : &[&[MemType]] = &[&[R, R], &[R,W], &[W, R], &[W,W]];

#[test]
fn any_address() {

}
