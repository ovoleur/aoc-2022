use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ").skip(1).step_by(2);

        Ok(Self {
            n: iter.next().ok_or(1)?.parse().map_err(|_| 4)?,
            from: iter.next().ok_or(2)?.parse::<usize>().map_err(|_| 5)? - 1,
            to: iter.next().ok_or(3)?.parse::<usize>().map_err(|_| 6)? - 1,
        })
    }
}

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<Stack>,
}

impl CrateStacks {
    fn new() -> Self {
        Self { stacks: vec![] }
    }

    fn add_line(mut self, chars: Vec<char>) -> Self {
        if self.stacks.is_empty() {
            self.stacks = Vec::with_capacity(chars.len());

            self.stacks
                .extend(chars.into_iter().map(|c| Stack::new().add_char(c)));
        } else {
            self.stacks = self
                .stacks
                .into_iter()
                .zip(chars.into_iter())
                .map(|(stack, c)| stack.add_char(c))
                .collect();
        }

        self
    }

    fn move_from_to_q1(&mut self, instruction: Instruction) {
        let Instruction { n, from, to } = instruction;

        let moved_content = self.stacks[from].remove_from_q1(n);
        self.stacks[to].add_to(moved_content);
    }

    fn move_from_to_q2(&mut self, instruction: Instruction) {
        let Instruction { n, from, to } = instruction;

        let moved_content = self.stacks[from].remove_from_q2(n);
        self.stacks[to].add_to(moved_content);
    }

    fn collect_char_on_top(&self) -> Vec<char> {
        self.stacks
            .iter()
            .flat_map(Stack::get_char_on_top)
            .collect()
    }
}

#[derive(Debug)]
struct Stack {
    content: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    fn add_char(mut self, c: char) -> Self {
        if c.is_uppercase() {
            self.content.insert(0, c);
        }
        self
    }

    fn get_char_on_top(&self) -> Option<char> {
        self.content.get(self.content.len() - 1).copied()
    }

    fn remove_from_q1(&mut self, n: usize) -> Vec<char> {
        let lower = self.content.len() - n;
        let mut ret: Vec<_> = self.content.drain(lower..self.content.len()).collect();

        ret.reverse();

        ret
    }

    fn remove_from_q2(&mut self, n: usize) -> Vec<char> {
        let lower = self.content.len() - n;
        self.content.drain(lower..self.content.len()).collect()
    }

    fn add_to(&mut self, new_content: Vec<char>) {
        self.content.extend(new_content);
    }
}

fn q1() {
    let procedure = include_str!("input.txt");

    let (schema, instructions) = procedure.split_once("\n\n").expect("woups");

    let mut crate_stack = schema
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .fold(CrateStacks::new(), CrateStacks::add_line);

    let parsed_instructions: Vec<_> = instructions
        .lines()
        .flat_map(Instruction::from_str)
        .collect();

    for instruction in parsed_instructions {
        crate_stack.move_from_to_q1(instruction);
    }

    println!(
        "q1 {:?}",
        crate_stack
            .collect_char_on_top()
            .iter()
            .map(char::to_string)
            .reduce(|a, b| a + &b)
            .expect("Answer 1")
    )
}

fn q2() {
    let procedure = include_str!("input.txt");

    let (schema, instructions) = procedure.split_once("\n\n").expect("woups");

    let mut crate_stack = schema
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .fold(CrateStacks::new(), CrateStacks::add_line);

    let parsed_instructions: Vec<_> = instructions
        .lines()
        .flat_map(Instruction::from_str)
        .collect();

    for instruction in parsed_instructions {
        crate_stack.move_from_to_q2(instruction);
    }

    println!(
        "q2 {:?}",
        crate_stack
            .collect_char_on_top()
            .iter()
            .map(char::to_string)
            .reduce(|a, b| a + &b)
            .expect("Answer 2")
    )
}

fn main() {
    q1();
    q2();
}
