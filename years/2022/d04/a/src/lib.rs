#![feature(test)]
extern crate test;

// 2-3,4-5
pub fn solution(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let p = line
                .split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|n| n.parse::<isize>().unwrap())

                    .collect::<Vec<_>>()
                })
            .collect::<Vec<_>>();

            let [a,b,c,d] = [p[0][0],p[0][1],p[1][0],p[1][1]];
            a >= c && b <= d || a <= c && b >= d
        })
        .count()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(2, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
