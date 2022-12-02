#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> isize {
    input
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|round| ((round[0] - b'A') as isize, (round[2] - b'X') as isize))
        .map(|(a, b)| 1 + b + 3 * ((1 + b - a) % 3).abs())
        .sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(15, solution(input));
}
#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
