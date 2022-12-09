#![feature(test)]
extern crate test;
use std::collections::HashSet;

type Pos = (isize, isize);

fn get_next_tail_pos(head: Pos, tail: Pos) -> Pos {
    let d = ((head.0 - tail.0), (head.1 - tail.1));
    if d.0.pow(2) + d.1.pow(2) <= 2 {
        return tail;
    }

    match d {
        (a, 0) if a > 1 => (tail.0 + 1, tail.1),
        (0, b) if b < -1 => (tail.0, tail.1 - 1),
        (a, 0) if a < -1 => (tail.0 - 1, tail.1),
        (0, b) if b > 1 => (tail.0, tail.1 + 1),

        (a, b) if a > 0 && b > 0 => (tail.0 + 1, tail.1 + 1),
        (a, b) if a > 0 && b < 0 => (tail.0 + 1, tail.1 - 1),
        (a, b) if a < 0 && b < 0 => (tail.0 - 1, tail.1 - 1),
        (a, b) if a < 0 && b > 0 => (tail.0 - 1, tail.1 + 1),
        _ => unreachable!(),
    }
}

pub fn solution(input: &str) -> usize {
    let mut unique_tail_positions: HashSet<Pos> = HashSet::new();
    input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<usize>().unwrap()))
        .map(|(dir, steps)| std::iter::repeat(dir).take(steps).collect::<Vec<_>>())
        .flatten()
        .fold(((0, 0), (0, 0)), |(head, tail), direction| {
            let head = match direction {
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                _ => unreachable!(),
            };
            let tail = get_next_tail_pos(head, tail);
            unique_tail_positions.insert(tail);
            (head, tail)
        });
    unique_tail_positions.len()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(13, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
