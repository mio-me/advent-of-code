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
    let ymax = paths.iter().map(|p| p.start.y.max(p.end.y)).max().unwrap();

    let mut grid: Grid = (0..=(ymax+1))
        .into_iter()
        .map(|_| (0..1000).into_iter().map(|_| false).collect())
        .chain(std::iter::once(
            (0..1000).into_iter().map(|_| true).collect(),
        ))
        .collect();

    for path in paths {
        if path.start.y == path.end.y {
            (path.start.x.min(path.end.x)..=path.end.x.max(path.start.x)).for_each(|x| {
                grid[path.start.y][x] = true;
            });
        } else {
            (path.start.y.min(path.end.y)..=path.end.y.max(path.start.y)).for_each(|y| {
                grid[y][path.start.x] = true;
            });
        }
    }

    let next_step = |sand_grain: &Point, grid: &Grid| -> Option<Point> {
        [sand_grain.x, sand_grain.x - 1, sand_grain.x + 1]
        .iter()
        .find_map(|x| {
            if !grid[sand_grain.y + 1][*x] {
                Some(Point::new(*x, sand_grain.y + 1))
            } else {
                None
            }
        })
    };

    let mut count = 0;
    'sand_heap: loop {
        let mut p = Point::new(500, 0);
        'sand_grain: loop {
            match next_step(&p, &grid) {
                Some(next) => {
                    p = next;
                }
                None => {
                    if p.y == 0 {
                        break 'sand_heap;
                    } else {
                        break 'sand_grain;
                    }
                }
            }
        }
        grid[p.y][p.x] = true;
        count += 1;
    }
    count + 1
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
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
        let start = Point::new(start.0.parse().unwrap(), start.1.parse().unwrap());
        let end = Point::new(end.0.parse().unwrap(), end.1.parse().unwrap());
        Path { start, end }
    }
}

#[test]
fn example() {
    let input = include_str!("../../test_input.txt");
    assert_eq!(93, solution(input));
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = include_str!("../../input.txt");
    b.iter(|| solution(input));
    println!("Solution: {}", solution(input));
}
