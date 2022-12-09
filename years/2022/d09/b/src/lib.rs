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
    let mut initial_rope = (0..=9).into_iter().map(|_| (0, 0)).collect::<Vec<Pos>>();
    let mut unique_tail_positions: HashSet<(isize, isize)> = HashSet::new();
    input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<usize>().unwrap()))
        .map(|(dir, steps)| std::iter::repeat(dir).take(steps).collect::<Vec<_>>())
        .flatten()
        .fold(initial_rope, |mut rope, direction| {
            let head = rope[0];
            let head = match direction {
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                _ => unreachable!(),
            };
            {
                let old_head = rope.get_mut(0).unwrap();
                old_head.0 = head.0;
                old_head.1 = head.1;
            }
            for i in 1..=9 {
                let head = rope[i - 1];
                let tail = rope[i];

                let tail = get_next_tail_pos(head, tail);

                let old_tail = rope.get_mut(i).unwrap();
                old_tail.0 = tail.0;
                old_tail.1 = tail.1;

                unique_tail_positions.insert(tail);
            }
            rope
        });

    unique_tail_positions.len() + 1
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(36, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
