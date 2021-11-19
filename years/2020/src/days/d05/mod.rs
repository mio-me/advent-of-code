use crate::util;

pub fn solve() {
  util::solve("d05", p1, p2)
}

fn p1(input: &str) -> usize {
  input
    .as_bytes()
    .chunks(11)
    .map(|b| {
      b[..10]
        .iter()
        .fold(0, |id, b| (id << 1) | (!b & 4) as usize >> 2)
    })
    .max()
    .unwrap()
}
use std::collections::HashSet;
fn p2(input: &str) -> usize {
  let seats = input
    .as_bytes()
    .chunks(11)
    .map(|b| {
      b[..10]
        .iter()
        .fold(0, |id, b| (id << 1) | (!b & 4) as usize >> 2)
    })
    .collect::<HashSet<_>>();
  for seat in 1..904 {
    if !seats.contains(&seat) && seats.contains(&(seat - 1)) && seats.contains(&(seat + 1)) {
      return seat;
    }
  }
  0
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1("BFFFBBFRRR"), 567);
    assert_eq!(p1("FFFBBBFRRR"), 119);
    assert_eq!(p1("BBFFBBFRLL"), 820);
  }

  const DATA: &'static str = "BFFFBBFRRR
FFFBBBFRRR
FFFBBBFRLR
BBFFBBFRLL";

  #[test]
  fn s2() {
    assert_eq!(p2(DATA), 118);
  }
}
