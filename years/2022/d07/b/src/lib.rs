#![feature(test)]
extern crate test;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn solution(input: &str) -> isize {
    let mut map: HashMap<String, isize> = HashMap::new();
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
                            map.insert(path, *old + size.parse::<isize>().unwrap());
                        }
                    });
                }
            } // file add to map
        };
    });
    let needed_space = (40_000_000 - map.get("/").unwrap()).abs();
    map.values()
        .map(|size| (size - needed_space, *size))
        .filter(|n| n.0 >= 0)
        .fold((isize::MAX, 0), |(diff, size), next| {
            if next.0 < diff {
                next
            } else {
                (diff, size)
            }
        })
        .1
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(24933642, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
