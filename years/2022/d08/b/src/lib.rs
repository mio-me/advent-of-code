#![feature(test)]
extern crate test;

use std::collections::HashSet;
use std::iter;

#[derive(Debug, Clone, Copy)]
struct Tree {

    size: isize,
    x: usize,
    y: usize,
    score: usize
}

pub fn solution(input: &str) -> usize {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let len = first.len();
    let mut grid = iter::once(first)
        .chain(lines)
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, n)| Tree { size: atoi::ascii_to_digit::<isize>(n as u8).unwrap(), x, y, score: 1})
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    for start in 0..grid.len() {
        eprintln!("ITEM {:?}", grid[start]);
        let (_, x, y, _) = grid[start];
        walk_grid(&mut grid, (x, y), len, |i| y * len + i);
        walk_grid(&mut grid, (x, y + 1), len, |i| i * len + x);
        grid.reverse();
        walk_grid(&mut grid, (x, y), len, |i| y * len + i);
        walk_grid(&mut grid, (x, y), len, |i| i * len + x);
        grid.reverse();
    }

    *(grid.iter().map(|(_, _, _, score)| score).max().unwrap())
}

fn walk_grid<F>(grid: &mut Vec<Tree>, idx: (usize, usize), len: usize, mapping: F)
    where
        F: Fn(usize) -> usize,
{
    let mut visible = 0;
    let current = grid[idx.0];
    for i in idx.1..len {
        let tree = grid[mapping(i)];
        eprintln!("{:?}", tree);
        if current.0 > tree.0 {
            visible += 1;
        } else {
            grid.get_mut(idx.0).unwrap().3 *= visible;
            return;
        }
    }
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(8, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
