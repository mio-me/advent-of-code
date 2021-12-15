use crate::util;

pub fn solve() {
    util::solve("d07", p1, p2)
}

fn p1(input: &str) -> usize {
    let mut nums: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();

    nums.sort();
    let mut uniques = nums.clone();
    uniques.dedup();
    uniques
        .iter()
        .map(|n| nums.iter().map(|x| x.abs_diff(*n) as usize).sum())
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let sum = |n: usize| n * (n + 1) / 2;

    let mut nums: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();

    (nums.iter().sum::<i32>() / nums.len() as i32..)
        .take(2)
        .map(|t| {
            nums.iter()
                .map(|n| sum((n - t).abs() as usize))
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 37);
    }

    #[test]
    #[ignore]
    fn s2() {
        assert_eq!(p2(DATA), 168);
    }
}
