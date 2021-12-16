use crate::util;

pub fn solve() {
    util::solve("d08", p1, p2)
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_once('|')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .filter(|clock| match clock.chars().count() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut part = line.splitn(2, |&b| b == b'|');
            let mut input = part.next().unwrap().split(|&b| b == b' ');
            let one = input.clone().find(|d| d.len() == 2).unwrap();
            let four = input.find(|d| d.len() == 4).unwrap();
            part.next()
                .unwrap()
                .split(|&b| b == b' ')
                .skip(1)
                .map(|d| match d.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    len => match (
                        len,
                        d.iter().filter(|&b| one.contains(b)).count(),
                        d.iter().filter(|&b| four.contains(b)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                })
                .enumerate()
                .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
        })
        .sum::<u32>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    const DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 26);
    }

    #[test]
    fn s2() {
        assert_eq!(p2(DATA), 61229);
    }
}
