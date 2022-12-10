#![feature(test, array_windows)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input
        .as_bytes()
        .array_windows::<4>()
        .position(|[a, b, c, d]| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(7, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
