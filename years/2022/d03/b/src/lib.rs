#![feature(test, iter_array_chunks)]
extern crate test;

use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .array_chunks::<3>()
    .map(|arr| arr.map(|x| x.chars().collect::<HashSet<_>>()))
        .map(|[a, b, c]| {
            let ab = a.intersection(&b).map(|c| *c).collect::<HashSet<char>>();
            let badge =  ab.intersection(&c).collect::<String>().chars().take(1).last().unwrap();
            if badge.is_lowercase() {
                1 + badge as usize - 'a' as usize
            } else {
                27 + badge as usize - 'A' as usize
            }
        })
        .sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(70, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
