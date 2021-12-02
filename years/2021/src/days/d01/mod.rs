use crate::util;

pub fn solve() {
  util::solve("d01", p1, p2)
}

fn p1(input: &str) -> usize {
  input
    .lines()
    .map(|s| s.parse().unwrap())
    .collect::<Vec<usize>>()
    .array_windows()
    .filter(|[a,b]| a<b)
    .count()
}

fn p2(input: &str) -> usize {
  input
    .lines()
    .map(|s| s.parse().unwrap())
    .collect::<Vec<usize>>()
    .array_windows()
    .filter(|[a,_,_,b]| a < b)
    .count()
}


#[cfg(test)]
mod test {
  const DATA: &str = "199
200
208
210
200
207
240
269
260
263";

  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 7);
  }

  #[test]
  fn s2() {
    assert_eq!(p2(DATA), 5);
  }
}
