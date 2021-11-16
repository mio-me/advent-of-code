use crate::util::parse_file;
use std::collections::HashSet;

fn p1(data: &[i32], x: i32) -> Option<i32> {
  let mut matched: HashSet<i32> = HashSet::new();
  for n in data.into_iter() {
    if matched.contains(&n) {
      return Some(n * (x - n));
    } else {
      matched.insert(x - n);
    }
  }
  None
}

fn p2(data: &[i32]) -> Option<i32> {
  for n in data.into_iter() {
    if let Some(i) = p1(data, 2020 - n) {
      return Some(n * i);
    }
  }
  None
}

pub fn solve() {
  let input = &parse_file("d01");
  println!("Part 1: {:?}", p1(input, 2020).unwrap());
  println!("Part 2: {:?}", p2(input).unwrap());
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::util::parse_input;
  const DATA: &str = "1721
979
366
299
675
1456";

  #[test]
  fn s1() {
    assert_eq!(p1(&parse_input(DATA), 2020), Some(514579));
  }

  #[test]
  fn s2() {
    assert_eq!(p2(&parse_input(DATA)), Some(241861950));
  }
}
