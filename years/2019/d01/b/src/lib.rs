#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> isize {
    input
    .lines()
    .map(|l| {
        let mut mass = l.parse::<isize>().unwrap() / 3 - 2;
        let mut curr = mass;
        loop  {
            let next = curr / 3 - 2;
            if next < 0 {
                break;
            }
            mass += next;
            curr = next;
        }
        mass
    })
    .sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(51316, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
