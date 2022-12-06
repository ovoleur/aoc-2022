fn main() {
    let input = include_str!("input.txt");

    let mut calories: Vec<_> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|number_str| u64::from_str_radix(number_str, 10).unwrap())
                .sum::<u64>()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    println!("{}", calories.iter().take(3).sum::<u64>());
}
