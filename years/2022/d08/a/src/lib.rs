#![feature(test)]
extern crate test;
use std::collections::HashSet;
use std::iter;

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
                .map(|(x, n)| (atoi::ascii_to_digit::<isize>(n as u8).unwrap(), x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    walk_grid(&grid, len, &mut visible_trees, |i, j| i * len + j);
    walk_grid(&grid, len, &mut visible_trees, |i, j| i + len * j);

    grid.reverse();
    walk_grid(&grid, len, &mut visible_trees, |i, j| i * len + j);
    walk_grid(&grid, len, &mut visible_trees, |i, j| i + len * j);

    visible_trees.len()
}

fn walk_grid<F>(
    grid: &Vec<(isize, usize, usize)>,
    len: usize,
    visible_trees: &mut HashSet<(usize, usize)>,
    mapping: F,
) where
    F: Fn(usize, usize) -> usize,
{
    for i in 0..len {
        let mut max = isize::MIN;
        for j in 0..len {
            let tree = grid[mapping(i, j)];
            if tree.0 > max {
                max = tree.0;
                visible_trees.insert((tree.1, tree.2));
            }
        }
    }
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(21, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
