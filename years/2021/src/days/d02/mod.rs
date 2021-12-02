use crate::util;

pub fn solve() {
  util::solve("d02", p1, p2)
}

fn p1(input: &str) -> usize {
  let (x,y) = input
    .lines()
    .map(|line| line.split_once(" ").unwrap())
    .fold((0,0), |(x, y), (direction, length)| {
      match (direction, length.parse::<i32>().unwrap()) {
        ("up", length) => (x, y-length),
        ("down", length) => (x, y+length),
        ("forward", length) => (x+length, y),
        _ => panic!("invalid direction")
      }
    });
    (x*y).try_into().unwrap()
}

fn p2(input: &str) -> usize {
  let (x,y,_) = input
  .lines()
  .map(|line| line.split_once(" ").unwrap())
  .fold((0,0,0), |(x, y, aim), (direction, length)| {
    match (direction, length.parse::<i32>().unwrap()) {
      ("up", length) => (x, y, aim-length),
      ("down", length) => (x, y, aim+length),
      ("forward", length) => (x+length, y + aim * length, aim),
      _ => panic!("invalid direction")
    }
  });
  (x*y).try_into().unwrap()
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
