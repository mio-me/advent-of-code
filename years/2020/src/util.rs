use std::fs;

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split_whitespace()
    .map(|str| str.parse().unwrap())
    .collect()
}

pub fn parse_file(day: &str) -> Vec<i32> {
  let input = fs::read_to_string(format!("./src/days/{}/input.txt", day)).unwrap();
  parse_input(&input)
}
