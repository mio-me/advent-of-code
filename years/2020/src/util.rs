pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split_whitespace()
    .map(|str| str.parse().unwrap())
    .collect()
}
