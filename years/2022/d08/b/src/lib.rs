#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> usize {
    let d = input.as_bytes();
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    let g: Vec<&[u8]> = d.split(|b| b == &b'\n').collect();

    let mut max = 0;
    for x in 0..s {
        for y in 0..s {
            let cur = g[y][x];
            if cur <= 1 {
                continue;
            }
            max = max.max(
                ((1..y)
                    .map(|yy| g[y - yy][x])
                    .position(|h| h >= cur)
                    .unwrap_or_else(|| y.wrapping_sub(1))
                    .wrapping_add(1))
                    * ((y + 1..s)
                        .map(|y| g[y][x])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| (s - y).wrapping_sub(2))
                        .wrapping_add(1))
                    * ((1..x)
                        .map(|xx| g[y][x - xx])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| x.wrapping_sub(1))
                        .wrapping_add(1))
                    * ((x + 1..s)
                        .map(|x| g[y][x])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| (s - x).wrapping_sub(2))
                        .wrapping_add(1)),
            );
        }
    }
    max
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(8, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
