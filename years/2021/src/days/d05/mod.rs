use std::collections::HashSet;

use crate::util;

pub fn solve() {
    util::solve("d05", p1, p2)
}

fn p1(input: &str) -> usize {
    let mut first: HashSet<usize> = HashSet::new();
    let mut danger: HashSet<usize> = HashSet::new();

    input.lines().for_each(|line| {
        let (from, to) = line.split_once(" -> ").unwrap();
        let from = from.split_once(',').unwrap();
        let (x1, y1) = (
            from.0.parse::<usize>().unwrap(),
            from.1.parse::<usize>().unwrap(),
        );
        let to = to.split_once(',').unwrap();
        let (x2, y2) = (
            to.0.parse::<usize>().unwrap(),
            to.1.parse::<usize>().unwrap(),
        );
        if x1 != x2 && y1 != y2 {
            return;
        }
        let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));

        let mut mark = |x, y| {
            let num = (x * 1000 + 1000) + y;
            if first.contains(&num) {
                danger.insert(num);
            } else {
                first.insert(num);
            }
        };
        if x1 == x2 {
            (y1..=y2).for_each(|y| mark(x1, y));
        } else if y1 == y2 {
            (x1..=x2).for_each(|x| mark(x, y1));
        }
    });
    danger.iter().count()
}

fn p2(input: &str) -> usize {
    let mut first: HashSet<i32> = HashSet::new();
    let mut danger: HashSet<i32> = HashSet::new();

    input.lines().for_each(|line| {
        let (from, to) = line.split_once(" -> ").unwrap();
        let from = from.split_once(',').unwrap();
        let (x1, y1) = (
            from.0.parse::<i32>().unwrap(),
            from.1.parse::<i32>().unwrap(),
        );
        let to = to.split_once(',').unwrap();
        let (x2, y2) = (to.0.parse::<i32>().unwrap(), to.1.parse::<i32>().unwrap());

        let mut mark = |x, y| {
            let num = (x * 1000 + 1000) + y;
            if first.contains(&num) {
                danger.insert(num);
            } else {
                first.insert(num);
            }
        };

        if x1 != x2 && y1 != y2 {
            let xs = (x1 - x2).signum();
            let ys = (y1 - y2).signum();

            let mut i = x1;
            let mut j = y1;
            while i != x2 + (-1) * xs || j != y2 + (-1) * ys {
                mark(i, j);
                i = i + (-1) * xs;
                j = j + (-1) * ys;
            }
            return;
        }
        let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));

        if x1 == x2 {
            (y1..=y2).for_each(|y| mark(x1, y));
        } else if y1 == y2 {
            (x1..=x2).for_each(|x| mark(x, y1));
        }
    });
    danger.iter().count()
}

#[cfg(test)]
mod test {
    const DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 5);
    }

    #[test]
    fn s2() {
        assert_eq!(p2(DATA), 12);
    }
}
