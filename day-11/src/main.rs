use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Squared,
}

#[derive(Debug)]
struct Monkey {
    holdings: Vec<usize>,
    operation: Operation,
    test_divisibility_by: usize,
    to_monkey_true: usize,
    to_monkey_false: usize,

    inspect_count: usize,
}

impl Monkey {
    fn add(&mut self, item: usize) {
        self.holdings.push(item);
    }
}

impl FromStr for Monkey {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().skip(1);
        let holdings: Vec<_> = iter
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .into_iter()
            .flat_map(|number| number.parse::<usize>())
            .collect();

        let mut operation_iter = iter
            .next()
            .unwrap()
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .split(" ")
            .skip(2);

        let operator = operation_iter.next().unwrap();
        let by = operation_iter.next().unwrap();

        let operation = match (operator, by) {
            ("*", "old") => Operation::Squared,
            ("*", _) => Operation::Multiply(by.parse().unwrap()),
            ("+", _) => Operation::Add(by.parse().unwrap()),
            _ => unreachable!("Or so it seems"),
        };

        let test_divisibility_by = iter
            .next()
            .unwrap()
            .split("by ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let to_monkey_true = iter
            .next()
            .unwrap()
            .split("to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let to_monkey_false = iter
            .next()
            .unwrap()
            .split("to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self {
            holdings,
            operation,
            test_divisibility_by,
            to_monkey_true,
            to_monkey_false,
            inspect_count: 0,
        })
    }
}

fn q1() {
    fn inspect(monkeys: &mut Vec<Monkey>) {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut send_to = vec![];

            for mut holding in monkey.holdings.drain(..) {
                monkey.inspect_count += 1;

                holding = match monkey.operation {
                    Operation::Add(by) => holding + by,
                    Operation::Multiply(by) => holding * by,
                    Operation::Squared => holding * holding,
                };
                holding /= 3;

                if holding % monkey.test_divisibility_by == 0 {
                    send_to.push((monkey.to_monkey_true, holding));
                } else {
                    send_to.push((monkey.to_monkey_false, holding));
                };
            }

            for (to_monkey, item) in send_to {
                monkeys[to_monkey].add(item);
            }
        }
    }

    let monkeys = include_str!("input.txt");

    let mut monkeys = monkeys
        .split("\n\n")
        .flat_map(|monkey| monkey.parse::<Monkey>())
        .collect::<Vec<_>>();

    for _ in 0..20 {
        inspect(&mut monkeys);
    }

    monkeys.sort_by(|monkey_a, monkey_b| monkey_b.inspect_count.cmp(&monkey_a.inspect_count));

    let res = monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("q1 : {res}")
}

fn q2() {
    fn inspect(monkeys: &mut Vec<Monkey>, common_multiplier: usize) {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut send_to = vec![];

            for mut holding in monkey.holdings.drain(..) {
                monkey.inspect_count += 1;
                holding = match monkey.operation {
                    Operation::Add(by) => holding + by,
                    Operation::Multiply(by) => holding * by,
                    Operation::Squared => holding * holding,
                };
                if holding > common_multiplier {
                    holding %= common_multiplier
                }

                if holding % monkey.test_divisibility_by == 0 {
                    send_to.push((monkey.to_monkey_true, holding));
                } else {
                    send_to.push((monkey.to_monkey_false, holding));
                };
            }

            for (to_monkey, item) in send_to {
                monkeys[to_monkey].add(item);
            }
        }
    }

    let monkeys = include_str!("input.txt");

    let mut monkeys = monkeys
        .split("\n\n")
        .flat_map(|monkey| monkey.parse::<Monkey>())
        .collect::<Vec<_>>();

    let common_multiplier = monkeys
        .iter()
        .map(|monke| monke.test_divisibility_by)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..10_000 {
        inspect(&mut monkeys, common_multiplier);
    }

    monkeys.sort_by(|monkey_a, monkey_b| monkey_b.inspect_count.cmp(&monkey_a.inspect_count));

    let res = monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("q2 : {res}")
}

fn main() {
    q1();
    q2();
}
