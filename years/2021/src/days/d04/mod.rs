use crate::util;
const WIDTH: usize = 5;
pub fn solve() {
    util::solve("d04", p1, p2)
}

fn p1(input: &str) -> usize {
    let mut input = input.split("\n\n");
    // extrahiere gezogene Zahlen
    let nums = input
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect::<Vec<usize>>();
    // get boards
    let boards = input
        .map(|board| {
            board
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for i in 4..nums.len() - 1 {
        for board in &boards {
            if let Some(x) = is_bingo(&nums[0..i], &board) {
                return x * nums[i - 1];
            }
        }
    }
    0
}

fn is_bingo(nums: &[usize], board: &Vec<usize>) -> Option<usize> {
    for i in 0..WIDTH {
        let len = i * WIDTH;
        if board[len..len + WIDTH]
            .iter()
            .all(|n| nums.iter().any(|num| num == n))
            || board
                .iter()
                .enumerate()
                .filter(|(j, _)| j % WIDTH == i)
                .all(|(_, n)| nums.iter().any(|num| num == n))
        {
            return Some(board.iter().sum::<usize>() - nums.iter().sum::<usize>());
        }
    }
    None
}

fn p2(input: &str) -> usize {
    let (nums, boards) = input.split_once("\n\n").unwrap();

    let nums = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    boards
        .split("\n\n")
        .map(|b| {
            let board = b.split_whitespace().map(|n| n.parse().unwrap()).collect();
            for i in 4..nums.len() - 1 {
                if let Some(x) = is_bingo(&nums[0..i], &board) {
                    return (i, x * nums[i - 1]);
                }
            }
            (0, 0)
        })
        .fold((0, 0), |(a, b), (c, d)| match a > c {
            true => (a, b),
            false => (c, d),
        })
        .1
}

#[cfg(test)]
mod test {
    const DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    const DATA_COL: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

2 22 18 10 14
0 11 8 16 21
12 13 23 15 17
3 6 26 9 24
7 5 20 19 4";

    const DATA_COL2: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

2 14 18 10 22
0 21 8 16 11
12 17 23 15 13
3 24 26 9 6
7 4 20 19 5

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 4512);
        assert_eq!(p1(DATA_COL), 4512);
        assert_eq!(p1(DATA_COL2), 4512);
    }

    #[test]
    fn s2() {
        assert_eq!(p2(DATA), 1924);
    }
}
