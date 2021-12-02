use crate::util;
use regex::Regex;

pub fn solve() {
  util::solve("d02", p1, p2)
}

fn p1(input: &str) -> usize {
  let mut x:i32 = 0;
  let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
  let y:i32 = input
    .lines()
    .map(|e| {
      let res = re.captures(e).unwrap();
      match &res[1] {
        "up" => -1*res[2].parse::<i32>().unwrap(),
        "down" => res[2].parse::<i32>().unwrap(),
        _ => {x += res[2].parse::<i32>().unwrap(); 0},
      }
    })
    .sum();
    (y*x).try_into().unwrap()
}

fn p2(input: &str) -> usize {
  let mut x:i32 = 0;
  let mut aim:i32 = 0;
  let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
  let y:i32 = input
    .lines()
    .map(|e| {
      let res = re.captures(e).unwrap();
      match &res[1] {
        "up" => {aim -= res[2].parse::<i32>().unwrap(); 0},
        "down" => {aim += res[2].parse::<i32>().unwrap(); 0},
        _ => {let factor =  res[2].parse::<i32>().unwrap(); x += factor; factor*aim},
      }
    })
    .sum();
    (y*x).try_into().unwrap()
}


#[cfg(test)]
mod test {
  const DATA: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
  use super::*;
  #[test]
  fn s1() {
    assert_eq!(p1(DATA), 150);
  }

  #[test]
  fn s2() {
    assert_eq!(p2(DATA), 900);
  }
}
