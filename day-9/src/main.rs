use std::collections::HashSet;

fn q1() {
    #[derive(Debug)]
    struct HeadAndTail {
        head: (isize, isize),
        tail: (isize, isize),
        tail_visited_position: HashSet<(isize, isize)>,
    }

    impl Default for HeadAndTail {
        fn default() -> Self {
            Self {
                head: (0, 0),
                tail: (0, 0),
                tail_visited_position: HashSet::from([(0, 0)]),
            }
        }
    }

    impl HeadAndTail {
        fn move_head(&mut self, direction: char) {
            match direction {
                'R' => {
                    self.head.0 += 1;
                }
                'L' => {
                    self.head.0 -= 1;
                }
                'U' => {
                    self.head.1 += 1;
                }
                'D' => {
                    self.head.1 -= 1;
                }
                _ => unreachable!("AOC says so"),
            }
            self.move_tail_if_needed()
        }

        fn is_tail_touching_head(&self) -> bool {
            let (head_x, head_y) = self.head;
            let (tail_x, tail_y) = self.tail;

            (-1..=1).into_iter().any(|i| {
                let tail_x = tail_x + i;
                (-1..=1).into_iter().any(|j| {
                    let tail_y = tail_y + j;
                    head_x == tail_x && head_y == tail_y
                })
            })
        }

        fn move_tail(&mut self) {
            let (head_x, head_y) = self.head;
            let (tail_x, tail_y) = self.tail;

            if head_y > tail_y {
                self.tail.1 += 1;
            } else if head_y < tail_y {
                self.tail.1 -= 1;
            }
            if head_x > tail_x {
                self.tail.0 += 1;
            } else if head_x < tail_x {
                self.tail.0 -= 1;
            }

            self.tail_visited_position.insert(self.tail);
        }

        fn move_tail_if_needed(&mut self) {
            if !self.is_tail_touching_head() {
                self.move_tail()
            }
        }
    }
    let mut h_and_t = HeadAndTail::default();

    let input = include_str!("input.txt");
    for line in input.lines() {
        let (direction, nb) = line.split_once(" ").expect("I believe");
        let direction = direction.chars().next().unwrap();
        let nb: usize = nb.parse().unwrap();
        for _ in 0..nb {
            h_and_t.move_head(direction);
        }
    }

    println!("q1 : {}", h_and_t.tail_visited_position.len());
}

fn q2() {
    const NB_KNOTS: usize = 9;

    #[derive(Debug)]
    struct HeadAndTail {
        head: (isize, isize),
        tail: [(isize, isize); 9],
        tail_visited_position: HashSet<(isize, isize)>,
    }

    impl Default for HeadAndTail {
        fn default() -> Self {
            Self {
                head: (0, 0),
                tail: (0..NB_KNOTS)
                    .into_iter()
                    .map(|_| (0, 0))
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .unwrap(),
                tail_visited_position: HashSet::from([(0, 0)]),
            }
        }
    }

    impl HeadAndTail {
        fn move_head(&mut self, direction: char) {
            match direction {
                'R' => {
                    self.head.0 += 1;
                }
                'L' => {
                    self.head.0 -= 1;
                }
                'U' => {
                    self.head.1 += 1;
                }
                'D' => {
                    self.head.1 -= 1;
                }
                _ => unreachable!("AOC says so"),
            }
            self.move_tail_if_needed();
        }

        fn insert_position_of_the_last(&mut self) {
            self.tail_visited_position.insert(self.tail[NB_KNOTS - 1]);
        }

        fn is_tail_touching_head(&self, idx: usize) -> bool {
            let (head_x, head_y) = if idx == 0 {
                self.head
            } else {
                self.tail[idx - 1]
            };
            let (tail_x, tail_y) = self.tail[idx];

            (-1..=1).into_iter().any(|i| {
                let tail_x = tail_x + i;
                (-1..=1).into_iter().any(|j| {
                    let tail_y = tail_y + j;
                    head_x == tail_x && head_y == tail_y
                })
            })
        }

        fn move_tail(&mut self, idx: usize) {
            let (head_x, head_y) = if idx == 0 {
                self.head
            } else {
                self.tail[idx - 1]
            };
            let (tail_x, tail_y) = self.tail[idx];

            if head_y > tail_y {
                self.tail[idx].1 += 1;
            } else if head_y < tail_y {
                self.tail[idx].1 -= 1;
            }
            if head_x > tail_x {
                self.tail[idx].0 += 1;
            } else if head_x < tail_x {
                self.tail[idx].0 -= 1;
            }
        }

        fn move_tail_if_needed(&mut self) {
            for i in 0..NB_KNOTS {
                if !self.is_tail_touching_head(i) {
                    self.move_tail(i)
                }
            }
            self.insert_position_of_the_last();
        }
    }
    let mut h_and_t = HeadAndTail::default();

    let input = include_str!("input.txt");
    for line in input.lines() {
        let (direction, nb) = line.split_once(" ").expect("I believe");
        let direction = direction.chars().next().unwrap();
        let nb: usize = nb.parse().unwrap();
        for _ in 0..nb {
            h_and_t.move_head(direction);
        }
    }

    println!("q2 : {}", h_and_t.tail_visited_position.len());
}

fn main() {
    q1();
    q2();
}
