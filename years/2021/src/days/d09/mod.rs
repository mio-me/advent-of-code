use crate::util;

pub fn solve() {
    util::solve("d09", p1, p2)
}

fn p1(input: &str) -> usize {
    let map = input.as_bytes().split(|&b| b == b'\n').collect::<Vec<_>>();
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                map.get((y as isize + yy) as usize)
                    .and_then(|l| l.get((x as isize + xx) as usize))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                sum += (cell - b'0') as usize + 1;
            }
        }
    }
    sum
}

fn p2(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod test {
    const DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    use super::*;
    // #[test]
    fn s1() {
        assert_eq!(p1(DATA), 15);
    }

    #[test]
    #[ignore]
    fn s2() {
        assert_eq!(p2(DATA), 4);
    }
}
