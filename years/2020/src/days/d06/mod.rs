use crate::util;

pub fn solve() {
  util::solve("d06", p1, p2)
}

use std::collections::HashSet;

fn p1(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|group| {
      group
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<_>>()
        .len()
    })
    .sum()
}

fn p2(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|group| {
      let group = group.lines().collect::<Vec<_>>();
      group
        .get(0)
        .unwrap()
        .chars()
        .filter(|c| group.iter().all(|g| g.contains(*c)))
        .count()
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;
  const DATA: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 11);
  }

  #[test]
  fn s2() {
    assert_eq!(p2(DATA), 6);
  }
}
