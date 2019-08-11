use std::iter::once;

pub fn bell(v: &Vec<usize>) -> Vec<Vec<usize>> {
  let max = v.iter().max().map(|&u| u + 1).unwrap_or(0);
  v.iter().chain(once(&max)).map(|&end| {
    let mut next = v.clone();
    next.push(end);
    next
  }).collect()
}

#[test]
fn test() {
  assert_eq!(bell(&vec!(0,1)), vec!(vec!(0,1,0), vec!(0,1,1), vec!(0,1,2)));
  assert_eq!(bell(&vec!()), vec!(vec!(0)));
}

pub fn unique_points(n: usize) -> Vec<Vec<usize>> {
  fn helper(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec!(vec!()) }
    helper(n-1)
      .into_iter()
      .flat_map(|poss| bell(&poss))
      .collect()
  }
  let mut out = helper(n);
  out.sort_unstable();
  out.dedup();
  out
}

#[test]
fn test_unique_assn() {
  assert_eq!(unique_points(3).len(), 5);
  assert_eq!(unique_points(4).len(), 15);
}
