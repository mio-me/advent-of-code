use crate::util;

pub fn solve() {
  util::solve("dx", p1, p2)
}

fn p1(input: &str) -> usize {
  input
    .lines()
    .count()
}

fn p2(input: &str) -> usize {
  input
    .lines()
    .count()
}


#[cfg(test)]
mod test {
  const DATA: &str = "";
  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 2);
  }

  #[test]
  fn s2() {
    assert_eq!(p2(DATA), 4);
  }
}
