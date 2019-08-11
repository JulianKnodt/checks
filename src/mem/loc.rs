
/// Loc is a representation of some location in memory,
/// with such things as a cache etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Loc {
  Addr(usize),
}
