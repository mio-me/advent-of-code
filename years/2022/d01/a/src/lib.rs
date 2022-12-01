#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .fold((0, 0), |(curr, max), line| {
            if line.is_empty() {
                (0, max.max(curr))
            } else {
                let curr = curr + line.parse::<usize>().unwrap();
                (curr, max.max(curr))
            }
        })
        .1
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(24000, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
