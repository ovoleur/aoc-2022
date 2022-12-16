use std::collections::HashSet;

fn q1() {
    const MOVES: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

    fn get_answer(
        puzzle: &[Vec<usize>],
        start: (usize, usize),
        destination: (usize, usize),
        path: &mut HashSet<(usize, usize)>,
        answers: &mut HashSet<usize>,
    ) -> Option<usize> {
        if answers
            .iter()
            .any(|&possible_answer| possible_answer < path.len())
        {
            return None;
        }

        let (start_x, start_y) = start;
        let (destination_x, destination_y) = destination;

        let mut legit_moves: Vec<_> = MOVES
            .iter()
            .filter_map(|&(move_x, move_y)| {
                let new_x = move_x + (start_x as isize);
                let new_y = move_y + (start_y as isize);

                // if new_x < 0
                //     || new_x >= puzzle.len() as isize
                //     || new_y < 0
                //     || new_y >= puzzle[new_x as usize].len() as isize
                // {
                //     return None;
                // }

                let Some(row) = puzzle.get(new_x as usize) else {
                    return None;
                };
                let Some(&new_height) = row.get(new_y as usize) else {
                    return None;
                };

                let ret = (new_x as usize, new_y as usize);
                if new_height <= puzzle[start_x][start_y] + 1 && !path.contains(&ret) {
                    Some(ret)
                } else {
                    None
                }
            })
            .collect();

        legit_moves.sort_by(|&(x_a, y_a), &(x_b, y_b)| {
            let distance_a =
                (destination_x as isize - x_a as isize) + (destination_y as isize - y_a as isize);
            let distance_b =
                (destination_x as isize - x_b as isize) + (destination_y as isize - y_b as isize);
            // println!(
            //     "{} vs {} : {:?}",
            //     distance_a,
            //     distance_b,
            //     distance_a.cmp(&distance_b)
            // );

            distance_b.cmp(&distance_a)
        });

        // Can end
        if legit_moves
            .iter()
            .any(|&(move_x, move_y)| move_x == destination_x && move_y == destination_y)
        {
            println!("return {}", path.len());
            return Some(path.len());
        }

        println!("Move to try : {legit_moves:?}");
        for move_to_try in legit_moves {
            path.insert(move_to_try);
            if let Some(res) = get_answer(puzzle, move_to_try, destination, path, answers) {
                answers.insert(res);
            }
            path.remove(&move_to_try);
        }

        answers.iter().min().copied()
    }

    let input = include_str!("input.txt");

    let mut start = (0, 0);
    let mut destination = (0, 0);

    let puzzle: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    'S' => {
                        start = (x, y);
                        'a' as usize
                    }
                    'E' => {
                        destination = (x, y);
                        'z' as usize
                    }
                    _ => c as usize,
                })
                .collect()
        })
        .collect();

    let answer = get_answer(
        &puzzle,
        start,
        destination,
        &mut HashSet::from([start]),
        &mut HashSet::new(),
    );

    println!("q1 : {answer:?}");
}

fn main() {
    q1();
}
