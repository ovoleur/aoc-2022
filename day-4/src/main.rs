fn line_to_digits_pair(line: &str) -> [[u32; 2]; 2] {
    line.split(',')
        .map(|range| {
            let mut digits = range.split('-');

            let lower: u32 = digits
                .next()
                .expect("First digit")
                .parse()
                .expect("Parse first digit");

            let upper: u32 = digits
                .next()
                .expect("Second digit")
                .parse()
                .expect("Parse second digit");

            [lower, upper]
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn fully_overlap(ranges: &[[u32; 2]; 2]) -> bool {
    let [[first_lower, first_upper], [second_lower, second_upper]] = ranges;
    (first_lower >= second_lower && first_upper <= second_upper)
        || (second_lower >= first_lower && second_upper <= first_upper)
}

fn has_one_in_common(ranges: &[[u32; 2]; 2]) -> bool {
    let [[first_lower, first_upper], [second_lower, second_upper]] = ranges;

    (first_lower >= second_lower && first_lower <= second_upper)
        || (first_upper >= second_lower && first_upper <= second_upper)
        || (second_lower >= first_lower && second_lower <= first_upper)
        || (second_upper >= first_lower && second_upper <= first_upper)
}

fn q1() {
    let planning = include_str!("input.txt");

    let overlapping_pairs = planning
        .lines()
        .map(line_to_digits_pair)
        .filter(fully_overlap)
        .count();

    println!("Q1 Overlapping Pairs : {overlapping_pairs:?}")
}

fn q2() {
    let planning = include_str!("input.txt");

    let overlapping_pairs = planning
        .lines()
        .map(line_to_digits_pair)
        .filter(has_one_in_common)
        .count();

    println!("Q2 Overlapping Pairs : {overlapping_pairs:?}")
}

fn main() {
    q1();
    q2();
}
