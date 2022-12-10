#![feature(test, array_windows)]
extern crate test;

pub fn solution(input: &str) -> usize {
    input
        .as_bytes()
        .array_windows::<14>()
        .position(|arr| {
            let mut set = 0u32;
            for e in 0..14 {
                let mask = 1 << (arr[e] - b'a');
                if  set & mask > 0 {
                    return false;
                } else {
                    set |= mask;
                }
            }
            true
        })
        .unwrap()
        + 14
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(19, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
