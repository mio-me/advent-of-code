#![feature(test, array_windows)]
extern crate test;
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let mut set = HashSet::with_capacity(14);
    input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|(_, arr)| {
            &set.clear();
            for e in *arr {
                if set.contains(e) {
                    return false;
                } else {
                    set.insert(e);
                }
            }
            true
        })
        .unwrap()
        .0
        + 14
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(19, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
