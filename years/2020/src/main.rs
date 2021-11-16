mod days;
mod util;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(format!("./input/d{}.txt", args[1])).unwrap();
    let input = util::parse_input(&data);
    days::d01::solve(&input);
}
