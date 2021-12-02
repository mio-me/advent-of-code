#![feature(array_windows)]

mod days;
mod util;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].parse().unwrap() {
        1 => days::d01::solve(),
        2 => days::d02::solve(),
        _ => println!("Day not found.."),
    }
}