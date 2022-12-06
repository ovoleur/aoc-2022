fn find_common_letter(input: (&str, &str)) -> char {
    let (first, second) = input;

    first
        .chars()
        .find(|c| second.contains(*c))
        .expect("Not quite right")
}

fn find_common_letter_q2(input: ((&str, &str), &str)) -> char {
    let ((first, second), third) = input;

    first
        .chars()
        .find(|c| second.contains(*c) && third.contains(*c))
        .expect("Not quite right")
}

fn get_score_from_char(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        c as u32 - 96
    } else if c >= 'A' && c <= 'Z' {
        c as u32 - 38
    } else {
        unreachable!("Yes but no")
    }
}

fn q1() {
    let rucksacks_content = include_str!("input.txt");

    let sum = rucksacks_content
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(find_common_letter)
        .map(get_score_from_char)
        .sum::<u32>();

    println!("Q1 Score : {sum}");
}

fn q2() {
    let rucksacks_content = include_str!("input.txt");

    let sum = rucksacks_content
        .lines()
        .step_by(3)
        .zip(rucksacks_content.lines().skip(1).step_by(3))
        .zip(rucksacks_content.lines().skip(2).step_by(3))
        .map(find_common_letter_q2)
        .map(get_score_from_char)
        .sum::<u32>();

    println!("Q2 Score : {sum}");
}

fn main() {
    q1();
    q2();
}
