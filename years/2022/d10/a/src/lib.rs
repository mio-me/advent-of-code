#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> i32 {
    let mut x = 1;
    let mut sum = 0;
    let mut cycle = 0;
    for line in input.lines() {
        cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += x*cycle;
        }

        if line != "noop" {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum += x*cycle;
            }
            let (_, inc) = line.split_once(" ").unwrap();
            x += inc.parse::<i32>().unwrap();
        }
    }
    sum
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(13140, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
