use crate::util;

pub fn solve() {
  util::solve("d03", p1, p2);
}

fn p1(input: &str) -> usize {
  count_trees(input, 1, 3)
}

fn count_trees(input: &str, a: usize, b: usize) -> usize {
  input
    .lines()
    .enumerate()
    .filter(|(i, line)| {
      if i % a != 0 {
        return false;
      }
      let len = (*line).len();
      match len <= i * b {
        true => (*line).chars().nth((i / a * b) % len) == Some('#'),
        false => (*line).chars().nth(i / a * b) == Some('#'),
      }
    })
    .count()
}

fn p2(input: &str) -> usize {
  count_trees(input, 1, 1)
    * count_trees(input, 1, 3)
    * count_trees(input, 1, 5)
    * count_trees(input, 1, 7)
    * count_trees(input, 2, 1)
}

#[cfg(test)]
mod test {
  use super::*;

  const DATA: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

  #[test]
  fn s1() {
    assert_eq!(7, p1(DATA));
  }

  #[test]
  fn s2() {
    assert_eq!(336, p2(DATA));
  }
}
