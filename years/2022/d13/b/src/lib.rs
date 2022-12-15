#![feature(test)]
extern crate test;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, map_opt},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::Ordering;

pub fn solution(input: &str) -> usize {
    let first = Entry::List(vec![Entry::List(vec![Entry::Int(2)])]);
    let second = Entry::List(vec![Entry::List(vec![Entry::Int(6)])]);
    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| entry(l.as_bytes()).unwrap().1)
        .chain(std::iter::once(first.clone()))
        .chain(std::iter::once(second.clone()))
        .collect::<Vec<_>>();
    lines.sort_unstable();

    (lines.iter().position(|e| e.cmp(&first) == Ordering::Equal).unwrap() + 1)
    * (lines.iter().position(|e| e.cmp(&second) == Ordering::Equal).unwrap() + 1)

}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Entry {
    Int(u8),
    List(Vec<Entry>),
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Entry::Int(left), Entry::Int(right)) => left.cmp(right),
            (Entry::List(left), Entry::List(right)) => match left.iter().cmp(right) {
                order if order != Ordering::Equal => order,
                _ => left.len().cmp(&right.len()),
            },
            (Entry::Int(_), Entry::List(right)) if right.len() == 1 => self.cmp(&right[0]),
            (Entry::Int(left), Entry::List(_)) => Entry::List(vec![Entry::Int(*left)]).cmp(other),
            (Entry::List(_), Entry::Int(_)) => other.cmp(self).reverse(),
        }
    }
}

fn entry(input: &[u8]) -> IResult<&[u8], Entry, ()> {
    alt((map(list, Entry::List), map(int, Entry::Int)))(input)
}

fn int(input: &[u8]) -> IResult<&[u8], u8, ()> {
    map_opt(nom::character::complete::digit1, atoi::atoi)(input)
}
fn list(input: &[u8]) -> IResult<&[u8], Vec<Entry>, ()> {
    delimited(char('['), separated_list0(char(','), entry), char(']'))(input)
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(140, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
