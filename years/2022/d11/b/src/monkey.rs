
pub struct Monkey {
    pub items: Vec<usize>,
    operation: Operation,
    test: Operation,
    pub prime: usize,
    pub business: usize,
}

impl Monkey {
    pub fn from_desc(input: [&[u8]; 6]) -> Self {
        let items = parse_starting_items(input[1]);
        let operation = parse_operation(input[2]);
        let (test, prime) = parse_test(input[3], input[4], input[5]);
        Monkey {
            items,
            operation,
            test,
            prime,
            business: 0,
        }
    }

    pub fn throw_items(&mut self, lcm: usize) -> Vec<(usize, usize)> {
        let thrown = self.items.iter().map(|item| {
            let worry_level = (&self.operation)(*item) % lcm;
            let next = (&self.test)(worry_level);
            self.business += 1;
            (next, worry_level)
        }).collect();
        self.items = vec![];
        thrown
    }
}

fn parse_starting_items(input: &[u8]) -> Vec<usize> {
    input[17..]
    .split(|b| b == &b',')
    .map(|n| atoi::atoi(&n[1..]).unwrap())
    .collect::<Vec<_>>()
}

type Operation = Box<dyn Fn(usize) -> usize>;

fn parse_operation(input: &[u8]) -> Operation {
    let factor: Option<usize> = atoi::atoi(&input[25..]);
    match (input[23], factor) {
        (b'*', Some(a)) => Box::new(move |x| x * a),
        (b'+', Some(a)) => Box::new(move |x| x + a),
        (b'*', None) => Box::new(move |x| x * x),
        (b'+', None) => Box::new(move |x| x + x),
        _ => unreachable!(),
    }
}

fn parse_test(divisor: &[u8], success: &[u8], failure: &[u8]) -> (Operation, usize) {
    let divisor: usize = atoi::atoi(&divisor[21..]).unwrap();
    let success: usize = atoi::atoi(&success[29..]).unwrap();
    let failure: usize = atoi::atoi(&failure[30..]).unwrap();
    (Box::new(move |x| if x % divisor == 0 { success } else { failure }), divisor)
}

#[test]
fn parsing_starting_items_line_works() {
    let t = parse_starting_items("  Starting items: 79, 98".as_bytes());
    assert_eq!(vec![79, 98], t);
}

#[test]
fn parsing_op_handles_all_combinations() {
    assert_eq!(
            190,
    parse_operation("  Operation: new = old * 19".as_bytes())(10)
    );
    assert_eq!(
            29,
    parse_operation("  Operation: new = old + 19".as_bytes())(10)
    );
    assert_eq!(
            20,
    parse_operation("  Operation: new = old + old".as_bytes())(10)
    );
    assert_eq!(
            100,
    parse_operation("  Operation: new = old * old".as_bytes())(10)
    );
}

#[test]
fn parsed_op_can_be_used_more_than_once() {
    let op = parse_operation("  Operation: new = old * 19".as_bytes());
    assert_eq!(190, op(10));
    assert_eq!(190, op(10));
}

#[test]
fn parsing_test_line_works() {
    let (t, divisor) = parse_test(
            "  Test: divisible by 23".as_bytes(),
    "    If true: throw to monkey 2".as_bytes(),
    "    If false: throw to monkey 3".as_bytes(),
    );
    assert_eq!(2, t(46));
    assert_eq!(3, t(47));
}
