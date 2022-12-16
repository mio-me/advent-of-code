#![feature(test)]
extern crate test;
use itertools::Itertools;

type Grid = Vec<Vec<bool>>;

pub fn solution(input: &str) -> usize {
    let paths = input
        .lines()
        .map(Path::from_desc)
        .flatten()
        .collect::<Vec<Path>>();

    let xmin = paths.iter().map(|p| p.start.x.min(p.end.x)).min().unwrap();
    let ymax = paths.iter().map(|p| p.start.y.max(p.end.y)).max().unwrap();
    let xmax = paths.iter().map(|p| p.start.x.max(p.end.x)).max().unwrap();

    let width = xmax - xmin;
    let start = Point {
        x: 500 - xmin,
        y: 0,
    };

    let mut grid: Grid = (0..=(ymax+1))
        .into_iter()
        .map(|_| (0..=(width+1)).into_iter().map(|_| false).collect())
        .collect();

    for path in paths {
        if path.start.y == path.end.y {
            (path.start.x.min(path.end.x)..=path.end.x.max(path.start.x)).for_each(|x| {
                grid[path.start.y][x - xmin] = true;
            });
        } else {
            (path.start.y.min(path.end.y)..=path.end.y.max(path.start.y)).for_each(|y| {
                grid[y][path.start.x - xmin] = true;
            });
        }
    }

    let next_step = |sand_grain: &Point, grid: &Grid| -> SimState {
        if sand_grain.y > ymax || sand_grain.x == 0 {
            SimState::Ended
        } else if !grid[sand_grain.y + 1][sand_grain.x] {
            SimState::Running(Some(Point {
                x: sand_grain.x,
                y: sand_grain.y + 1,
            }))
        } else if !grid[sand_grain.y + 1][sand_grain.x - 1] {
            SimState::Running(Some(Point {
                x: sand_grain.x - 1,
                y: sand_grain.y + 1,
            }))
        } else if !grid[sand_grain.y + 1][sand_grain.x + 1] {
            SimState::Running(Some(Point {
                x: sand_grain.x + 1,
                y: sand_grain.y + 1,
            }))
        } else {
            SimState::Running(None)
        }
    };

    let mut count = 0;
    'sand_heap: loop {
        let mut p = Point { x: 500 - xmin, y: 0 };
        'sand_grain: loop {
            match next_step(&p, &grid) {
                SimState::Running(Some(next)) => {
                    p = next;
                }
                SimState::Running(None) => break 'sand_grain,
                SimState::Ended => break 'sand_heap,
            }
        }
        grid[p.y][p.x] = true;
        count += 1;
    }
    count
}

enum SimState {
    Running(Option<Point>),
    Ended,
}

struct Point {
    x: usize,
    y: usize,
}
struct Path {
    start: Point,
    end: Point,
}
impl Path {
    fn from_desc(desc: &str) -> Vec<Self> {
        desc.split(" -> ")
            .tuple_windows::<(_, _)>()
            .map(Path::from_tuple)
            .collect()
    }

    fn from_tuple(desc: (&str, &str)) -> Self {
        let (start, end) = (
            desc.0.split_once(",").unwrap(),
            desc.1.split_once(",").unwrap(),
        );
        let start = Point {
            x: start.0.parse().unwrap(),
            y: start.1.parse().unwrap(),
        };
        let end = Point {
            x: end.0.parse().unwrap(),
            y: end.1.parse().unwrap(),
        };
        Path { start, end }
    }
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(24, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
