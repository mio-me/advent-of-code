#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (l, r) = line.split_once(",").unwrap();
            let [l, r] = [l, r].map(|elf| {
                let (start, end) = elf.split_once("-").unwrap();
                (
                    start.parse::<isize>().unwrap(),
                    end.parse::<isize>().unwrap(),
                )
            });
            (r.1 - l.0) * (l.1 - r.0) >= 0
        })
        .count()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(4, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
