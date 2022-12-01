#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> usize {
    let mut calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|n| n.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();
    calories.sort_unstable();
    calories.into_iter().rev().take(3).sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(45000, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
