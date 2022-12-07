#![feature(test, exclusive_range_pattern)]
extern crate test;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn solution(input: &str) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut working_dir: Vec<&str> = vec![];

    input.lines().for_each(|l| {
        match l.as_bytes() {
            _ls if l.starts_with("$ ls") => {} // ls skip
            _dir if l.starts_with("dir") => {} // dir skip
            [_dollar_sign, _, _c, _d, _, dir @ ..] if l.starts_with("$ cd") => {
                if dir == [b'.', b'.'] {
                    working_dir.pop();
                } else {
                    working_dir.push(from_utf8(dir).unwrap());
                    let key = working_dir.iter().map(|c| *c).collect::<String>();
                    map.insert(key, 0);
                }
            }
            _file => {
                if let Some((size, _)) = l.split_once(" ") {
                    (1..=working_dir.len()).for_each(|end| {
                        let path = working_dir.iter().take(end).map(|c| *c).collect::<String>();
                        if let Some(old) = map.get(&path) {
                            map.insert(path, *old + size.parse::<usize>().unwrap());
                        }
                    });
                }
            } // file add to map
        };
    });
    map.values().filter(|v| **v < 100000).sum()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(95437, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
