use crate::util;

pub fn solve() {
    util::solve("d10", p1, p2)
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::with_capacity(64);
            for c in line.bytes() {
                match c {
                    c if matches!(c, b'(' | b'{' | b'[' | b'<') => stack.push(c),
                    b')' => {
                        if stack.pop().unwrap() != b'(' {
                            return Some(b')');
                        }
                    }
                    c => {
                        if stack.pop().unwrap() != c - 2 {
                            return Some(c);
                        }
                    }
                }
            }
            None
        })
        .map(|e| [3, 25137, 57, 1197][e as usize / 30 - 1])
        .sum()
}

fn p2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::with_capacity(64);
            for c in line.bytes() {
                match c {
                    c if matches!(c, b'(' | b'{' | b'[' | b'<') => stack.push(c),
                    c => {
                        let cn = stack.pop().unwrap();
                        let x = (c as i32) - (cn as i32);
                        if x != 1 && x != 2 {
                            return None;
                        }
                    }
                }
            }

            Some(
                stack
                    .iter()
                    .rev()
                    .map(|e| [1, 4, 2, 3][*e as usize / 30 - 1])
                    .fold(0, |a, b| 5 * a + b),
            )
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    const DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    use super::*;
    #[test]
    fn s1() {
        assert_eq!(p1(DATA), 26397);
    }

    #[test]
    fn s2() {
        assert_eq!(p2(DATA), 288957);
    }
}
