#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input.lines().count()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(0, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
