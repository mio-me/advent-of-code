use std::fs;

pub fn read_input(day: &str) -> String {
    fs::read_to_string(format!("./src/days/{}/input.txt", day)).unwrap()
}
use std::time::Instant;

pub fn solve<F, G>(day: &str, p1: F, p2: G) -> ()
where
    F: Fn(&str) -> usize,
    G: Fn(&str) -> usize,
{
    let input = read_input(day);
    let now = Instant::now();
    let solution = p1(&input);
    let time = now.elapsed().as_micros();
    println!("Part 1: Solution: {} ; Time: {}", solution, time);
    let now = Instant::now();
    let solution = p2(&input);
    let time = now.elapsed().as_micros();
    println!("Part 2: Solution: {} ; Time: {}", solution, time);
}
