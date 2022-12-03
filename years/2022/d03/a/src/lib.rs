#![feature(test)]
extern crate test;

use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    input.lines().map(|l| {
        let (left, right) = l.split_at(l.len() /2);
        let left = left.chars().collect::<HashSet<_>>();
        let c = right.chars().find(|c| left.contains(c)).unwrap();
        if c.is_lowercase() {
            1 + c as usize - 'a' as usize
        } else {
            27 + c as usize - 'A' as usize
        }
    }).sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(157, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
