use crate::util;
use std::env;

pub fn solve() {
    util::solve("d03", p1, p2)
}

fn p1(input: &str) -> usize {
    let (width, half) = {
        match env::var("TEST") {
            Ok(_) => (5, 12 / 2),
            Err(_) => (12, 1000 / 2),
        }
    };
    let gamma = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .fold(vec![0; width], |sum, b| {
            sum.iter()
                .enumerate()
                .map(|(i, n)| n + ((b & 1 << i) >> i))
                .collect()
        })
        .iter()
        .enumerate()
        .map(|(i, n)| ((n >= &half) as usize) << i)
        .sum::<usize>();
    (gamma * (!gamma & ((1 << width) - 1))).into()
}
fn p2(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod test {

    use super::*;

    const DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    #[test]
    fn s1() {
        std::env::set_var("TEST", "TEST");
        assert_eq!(p1(DATA), 198);
    }

    // #[test]
    // fn s2() {
    //     assert_eq!(p2(DATA), 4);
    // }
}
