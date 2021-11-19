use crate::util;
use std::collections::{HashMap, HashSet};

pub fn solve() {
  util::solve("d04", p1, p2)
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

fn p2(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|entry| {
      entry
        .split_ascii_whitespace()
        .map(|field| field.split_once(':').unwrap())
        .collect::<HashMap<_, _>>()
    })
    .filter(|entry| REQ_P1.iter().all(|pair| entry.contains_key(pair)))
    .filter(|entry| entry.iter().all(|(key, value)| validate(key, value)))
    .count()
}

const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn validate(key: &str, value: &str) -> bool {
  match key {
    "byr" => value.parse::<usize>().unwrap().wrapping_sub(1920) <= 82,
    "iyr" => value.parse::<usize>().unwrap().wrapping_sub(2010) <= 10,
    "eyr" => value.parse::<usize>().unwrap().wrapping_sub(2020) <= 10,
    "hgt" => {
      if value.ends_with("cm") && value.len() == 5 {
        value[0..3].parse::<usize>().unwrap().wrapping_sub(150) <= 43
      } else if value.ends_with("in") && value.len() == 4 {
        value[0..2].parse::<usize>().unwrap().wrapping_sub(59) <= 27
      } else {
        false
      }
    }
    "hcl" => value.len() == 7,
    "ecl" => EYE_COLORS.iter().any(|v| v == &value),
    "pid" => value.len() == 9,
    "cid" => true,
    _ => panic!("unknown field type"),
  }
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

  const DATA_INVALID: &'static str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

  const DATA_VALID: &'static str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 2);
  }

  #[test]
  fn s2() {
    assert_eq!(p2(DATA_VALID), 4);
    assert_eq!(p2(DATA_INVALID), 0);
  }
}
