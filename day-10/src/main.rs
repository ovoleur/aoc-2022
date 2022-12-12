fn q1() {
    fn increment_cycle_and_add(cycle_number: &mut i32, solution: &mut i32, x: i32) {
        *cycle_number += 1;
        if *cycle_number == 20 || (*cycle_number + 20) % 40 == 0 {
            *solution += *cycle_number * x;
        }
    }

    let instructions = include_str!("input.txt");

    let mut solution = 0;
    let mut cycle_number = 0;
    let mut x = 1;

    for line in instructions.lines() {
        increment_cycle_and_add(&mut cycle_number, &mut solution, x);
        if line.starts_with("noop") {
            continue;
        }

        increment_cycle_and_add(&mut cycle_number, &mut solution, x);

        let (_, number) = line.split_once(" ").unwrap();
        let number: i32 = number.parse().unwrap();
        x += number;
    }

    println!("q1 : {solution}");
}

fn q2() {
    fn increment_cycle_and_add(cycle_number: &mut i32, solution: &mut String, x: i32) {
        *cycle_number += 1;

        let cycle_for_line = *cycle_number % 40;

        if x <= cycle_for_line && x + 3 > cycle_for_line {
            solution.push('#');
        } else {
            solution.push('.');
        }
        if cycle_for_line == 0 {
            solution.push('\n');
        }
    }

    let instructions = include_str!("input.txt");

    let mut solution = "".to_string();
    let mut cycle_number = 0;
    let mut x = 1;

    for line in instructions.lines() {
        increment_cycle_and_add(&mut cycle_number, &mut solution, x);
        if line.starts_with("noop") {
            continue;
        }

        increment_cycle_and_add(&mut cycle_number, &mut solution, x);

        let (_, number) = line.split_once(" ").unwrap();
        let number: i32 = number.parse().unwrap();
        x += number;
    }

    println!("q2 : \n{solution}");
}

fn main() {
    q1();
    q2();
}
