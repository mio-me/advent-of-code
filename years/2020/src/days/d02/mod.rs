use crate::util;

pub fn solve() {
  util::solve("d02", p1, p2)
}
use regex::Regex;

fn check_passwords<F>(input: &str, check: F) -> usize
where
  F: Fn(usize, usize, &str, &str) -> bool,
{
  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
  input
    .lines()
    .map(|line| re.captures(line).unwrap())
    .filter(|p| check(p[1].parse().unwrap(), p[2].parse().unwrap(), &p[3], &p[4]))
    .count()
}

fn p1(input: &str) -> usize {
  check_passwords(input, |min, max, c, pw| {
    let num = pw.matches(c).count();
    num >= min && num <= max
  })
}

fn p2(input: &str) -> usize {
  check_passwords(input, |a, b, c, pw| {
    let first = pw.chars().nth(a - 1);
    let second = pw.chars().nth(b - 1);
    (first == Some(c.parse().unwrap()) || second == Some(c.parse().unwrap())) && first != second
  })
}

#[cfg(test)]
mod test {
  use super::*;

  const DATA: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

  #[test]
  fn s1() {
    assert_eq!(2, p1(DATA))
  }

  #[test]
  fn s2() {
    assert_eq!(1, p2(DATA))
  }
}
