use std::collections::VecDeque;

use crate::util;

pub fn solve() {
    util::solve("d06", p1, p2)
}

fn p1(input: &str) -> usize {
    let mut fish = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    input.split(',').for_each(|x| fish[x.parse().unwrap()] += 1);

    (0..80).for_each(|_| {
        let new_fish = fish.pop_front().unwrap();
        fish[6] += new_fish;
        fish.push_back(new_fish);
    });
    fish.iter().sum()
}

fn p2(input: &str) -> usize {
    let mut fish = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    input.split(',').for_each(|x| fish[x.parse().unwrap()] += 1);

    (0..256).for_each(|_| {
        let new_fish = fish.pop_front().unwrap();
        fish[6] += new_fish;
        fish.push_back(new_fish);
    });
    fish.iter().sum()
}

#[cfg(test)]
mod test {
    const DATA: &str = "3,4,3,1,2";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 5934);
    }

    #[test]
    fn s2() {
        assert_eq!(p2(DATA), 26984457539);
    }
}
