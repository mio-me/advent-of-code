#![feature(test)]
extern crate test;

fn draw_to_screen(screen: &mut [[char; 40]; 6], cycle: usize, x: i32) {
    let xx = (cycle - 1) % 40;
    let y = (cycle - 1) / 40;
    if [x-1,x, x+1].contains(&(xx as i32)) {
        screen[y][xx] = '#';
    }
}

pub fn solution(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 0;
    let mut screen = [['.'; 40]; 6];
    for line in input.lines() {
        cycle += 1;
        draw_to_screen(&mut screen, cycle, x);
        if line != "noop" {
            cycle += 1;
            draw_to_screen(&mut screen, cycle, x);
            let (_, inc) = line.split_once(" ").unwrap();
            x += inc.parse::<i32>().unwrap();
        }
    }
    screen
        .map(|row| row.iter().chain(std::iter::once(&'\n')).collect::<String>())
        .iter()
        .map(|s| s.clone())
        .collect::<String>()
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    let output = include_str!("../test_output.txt");
    assert_eq!(output, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: \n{}", solution(input));
}
