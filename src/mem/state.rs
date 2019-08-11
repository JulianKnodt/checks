use std::collections::HashMap;
use crate::mem::Loc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State(HashMap<Loc, usize>);

impl State {
  pub fn empty() -> Self {
    State(HashMap::new())
  }
  pub fn new(defined: Vec<(Loc, usize)>) -> Self {
    State(defined.into_iter().collect())
  }
  pub fn new_combinations(def: &HashMap<Loc, Vec<usize>>) -> Vec<Self> {
    def.iter().fold(vec!(State::empty()), |acc, (l, vs)| {
      acc.into_iter().flat_map(|prev_state| {
        vs.iter().map(move |poss| {
          let mut new_state = prev_state.clone();
          new_state.0.insert(*l, *poss);
          new_state
        })
      }).collect()
    })
  }
  pub fn same_data(&self, l: &Loc, d: usize) -> bool {
    self.0.get(l).map_or(false, |data| data == &d)
  }
  pub fn same_opt_data(&self, l: Option<usize>, d: Option<usize>) -> bool {
    l.map_or(false, |l| self.0.get(&Loc::Addr(l)) == d.as_ref())
  }
}
