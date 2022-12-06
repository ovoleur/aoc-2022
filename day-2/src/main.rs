enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!("Not correct input"),
        }
    }
}

impl From<(&Shape, &str)> for Shape {
    fn from(value: (&Shape, &str)) -> Self {
        match value {
            (Self::Rock, "X") => Self::Scissors,
            (Self::Rock, "Y") => Self::Rock,
            (Self::Rock, "Z") => Self::Paper,
            (Self::Paper, "X") => Self::Rock,
            (Self::Paper, "Y") => Self::Paper,
            (Self::Paper, "Z") => Self::Scissors,
            (Self::Scissors, "X") => Self::Paper,
            (Self::Scissors, "Y") => Self::Scissors,
            (Self::Scissors, "Z") => Self::Rock,
            _ => unreachable!("Not correct input"),
        }
    }
}

impl Shape {
    fn get_game_score(&self, opponent: &Shape) -> u32 {
        let shape_score = match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let game_score = match self {
            Shape::Rock => match opponent {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match opponent {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match opponent {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        };

        shape_score + game_score
    }
}

fn q1() {
    let game_planned = include_str!("input.txt");

    let score = game_planned
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|letter| Shape::from(letter))
                .collect::<Vec<_>>()
        })
        .map(|one_game| {
            let opponent_shape = &one_game[0];
            let my_shape = &one_game[1];

            my_shape.get_game_score(opponent_shape)
        })
        .sum::<u32>();

    println!("Q1 Score : {score}");
}

fn q2() {
    let game_planned = include_str!("input.txt");

    let score = game_planned
        .lines()
        .map(|line| {
            let mut letters = line.split(" ");
            let opponent_shape = Shape::from(letters.next().expect("Oppenent letter"));
            let my_shape = Shape::from((&opponent_shape, letters.next().expect("My letter")));

            [opponent_shape, my_shape]
        })
        .map(|[opponent_shape, my_shape]| my_shape.get_game_score(&opponent_shape))
        .sum::<u32>();

    println!("Q2 Score : {score}");
}

fn main() {
    q1();
    q2();
}
