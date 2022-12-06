#![feature(test)]
extern crate test;

pub fn solution(input: &str) -> String {
    let (inital_state, instructions) = input.split_once("\n\n").unwrap();
    let mut initial_state = inital_state.lines().rev();

    let number_of_stacks = initial_state
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks);

    for _ in 0..number_of_stacks {
        stacks.push(vec![]);
    }

    initial_state.for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .for_each(|(i, c)| {
                stacks.get_mut(i / 4).unwrap().push(c);
            })
    });

    instructions.lines().for_each(|inst| {
        let idx = inst
            .split(" ")
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<_>>();

        for _ in 0..idx[0] {
            let to_insert = *(&stacks.get_mut(idx[1] - 1).unwrap().pop().unwrap());
            stacks.get_mut(idx[2] - 1).unwrap().push(to_insert);
        }
    });

    stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!("CMZ".to_owned(), solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
