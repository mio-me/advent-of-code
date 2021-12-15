#![feature(array_windows)]
#![feature(drain_filter)]
#![feature(int_abs_diff)]

mod days;
mod util;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].parse().unwrap() {
        1 => days::d01::solve(),
        2 => days::d02::solve(),
        3 => days::d03::solve(),
        4 => days::d04::solve(),
        5 => days::d05::solve(),
        6 => days::d06::solve(),
        7 => days::d07::solve(),
        _ => println!("Day not found.."),
    }
}
