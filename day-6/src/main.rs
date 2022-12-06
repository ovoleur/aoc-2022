const START_OF_PACKET: usize = 4;

fn q1() {
    let input = include_str!("input.txt");

    let response = input
        .chars()
        .collect::<Vec<char>>()
        .windows(START_OF_PACKET)
        .position(|chars| {
            chars
                .into_iter()
                .find(|c| chars.iter().filter(|inner| c == inner).count() > 1)
                .is_none()
        })
        .unwrap_or(0)
        + START_OF_PACKET;

    println!("Q1 : {response}");
}

const START_OF_MESSAGE: usize = 14;

fn q2() {
    let input = include_str!("input.txt");

    let response = input
        .chars()
        .collect::<Vec<char>>()
        .windows(START_OF_MESSAGE)
        .position(|chars| {
            chars
                .into_iter()
                .find(|c| chars.iter().filter(|inner| c == inner).count() > 1)
                .is_none()
        })
        .unwrap_or(0)
        + START_OF_MESSAGE;

    println!("Q1 : {response}");
}

fn main() {
    q1();
    q2();
}
