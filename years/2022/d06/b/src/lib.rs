#![feature(test, array_windows)]
extern crate test;

pub fn solution(input: &str) -> usize {
    let mut set = [0usize; 26];
    input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|(_, arr)| {
            for e in 0..14 {
                set[(arr[e] - b'a') as usize] += 1;
                if set[(arr[e] - b'a') as usize] > 1 {
                    (0..=e).for_each(|j| set[(arr[j] - b'a') as usize] = 0);
                    return false;
                }
            }
            true
        })
        .unwrap()
        .0
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
