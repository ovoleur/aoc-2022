fn q1() {
    fn is_tree_visible(x: usize, y: usize, height: u8, forest: &[Vec<u8>]) -> bool {
        // if edge -> visible
        let len_x = forest.len();
        let len_y = forest[x].len();

        (0..y).into_iter().all(|i| forest[x][i] < height)
            || (y + 1..len_y).into_iter().all(|i| forest[x][i] < height)
            || (0..x).into_iter().all(|i| forest[i][y] < height)
            || (x + 1..len_x).into_iter().all(|i| forest[i][y] < height)
    }

    let input = include_str!("input.txt");

    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree_heigh_str| (tree_heigh_str as u8 - '0' as u8))
                .collect()
        })
        .collect();

    let res = trees
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|&(y, &height)| is_tree_visible(x, y, height, &trees))
                .count()
        })
        .sum::<usize>();

    println!("q1 : {res}");
}

fn q2() {
    fn get_scenic_score(x: usize, y: usize, height: u8, forest: &[Vec<u8>]) -> usize {
        let len_x = forest.len();
        let len_y = forest[x].len();

        // if edge -> 0
        if y == 0 || x == 0 || x == len_x - 1 || y == len_y - 1 {
            return 0;
        }

        let mut left = 0;
        for i in (0..y).rev() {
            left += 1;
            if forest[x][i] >= height {
                break;
            }
        }

        let mut right = 0;
        for i in y + 1..len_y {
            right += 1;
            if forest[x][i] >= height {
                break;
            }
        }

        let mut up = 0;
        for i in (0..x).rev() {
            up += 1;
            if forest[i][y] >= height {
                break;
            }
        }

        let mut down = 0;
        for i in x + 1..len_x {
            down += 1;
            if forest[i][y] >= height {
                break;
            }
        }

        left * right * up * down
    }

    let input = include_str!("input.txt");

    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree_heigh_str| (tree_heigh_str as u8 - '0' as u8))
                .collect()
        })
        .collect();

    let res = trees
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, &height)| get_scenic_score(x, y, height, &trees))
                .max()
        })
        // .collect();
        .max()
        .unwrap();

    println!("q2 : {res:?}");
}

fn main() {
    q1();
    q2();
}
