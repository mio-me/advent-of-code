use crate::util;

pub fn solve() {
  let input = util::read_input("d02");
  println!("Part 1: {}", p1(&input));
  println!("Part 2: {}", p2(&input));
}
use regex::Regex;

fn check_passwords<F>(input: &str, check: F) -> u32
where
  F: Fn(usize, usize, char, &str) -> bool,
{
  let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
  input
    .split('\n')
    .map(|line| {
      let groups = re.captures_iter(line).next().unwrap();
      let a: usize = groups[1].parse().unwrap();
      let b: usize = groups[2].parse().unwrap();
      let c: char = groups[3].parse().unwrap();
      let pw = groups[4].to_owned();
      match check(a, b, c, &pw) {
        true => 1,
        false => 0,
      }
    })
    .fold(0, |a, b| a + b)
}

fn p1(input: &str) -> u32 {
  check_passwords(input, |min, max, c, pw| {
    let num = pw.matches(c).count();
    num >= min && num <= max
  })
}

fn p2(input: &str) -> u32 {
  check_passwords(input, |a, b, c, pw| {
    let first = pw.chars().nth(a - 1);
    let second = pw.chars().nth(b - 1);
    (first == Some(c) || second == Some(c)) && first != second
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
