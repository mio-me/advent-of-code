#![feature(test)]
extern crate test;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, map_opt},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::cmp::Ordering;

pub fn solution(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p| pair(p.as_bytes()).unwrap().1)
        .enumerate()
        .filter(|(_, (left, right))| left.cmp(right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

#[derive(PartialEq, Eq)]
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
fn pair(input: &[u8]) -> IResult<&[u8], (Entry, Entry), ()> {
    separated_pair(entry, tag("\n"), entry)(input)
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(13, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
