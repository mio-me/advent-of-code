#![feature(test, iter_array_chunks)]
extern crate test;
mod monkey;

use monkey::Monkey;

pub fn solution(input: &str) -> usize {
    let mut monkeys = input
    .as_bytes()
    .split(|b| b == &b'\n')
    .filter(|l| !l.is_empty())
    .array_chunks::<6>()
    .map(|desc| Monkey::from(desc))
    .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for (target, item) in monkeys[i].throw_items() {
                monkeys[target].items.push(item);
            }
        }
    }
    monkeys.sort_by(|a, b| b.business.partial_cmp(&a.business).unwrap());
    monkeys[0].business * monkeys[1].business
}


#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(10605, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}