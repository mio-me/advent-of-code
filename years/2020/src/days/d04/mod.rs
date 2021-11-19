use crate::util;
use regex::Regex;
use std::collections::HashSet;

pub fn solve() {
  util::solve("d04", p1, p1)
}
const REQ_P1: [&'static str; 7] = ["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid"];

fn p1(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|entry| {
      entry
        .split_ascii_whitespace()
        .map(|field| field.split(':').next().unwrap())
        .collect::<HashSet<_>>()
    })
    .filter(|entry| REQ_P1.iter().all(|value| entry.contains(value)))
    .count()
}

#[cfg(test)]
mod test {
  const DATA: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 2);
  }
}
