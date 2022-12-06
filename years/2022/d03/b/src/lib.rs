#![feature(test, iter_array_chunks)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input
        .as_bytes()
        .split(|b| *b == b'\n')
        .array_chunks::<3>()
        .filter_map(|[a, b, c]| a.iter().find(|x| b.contains(x) && c.contains(x)))
        .map(|badge| {
            if *badge >= b'a' {
                1 + (badge - b'a') as usize
            } else {
                27 + (badge - b'A') as usize
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
