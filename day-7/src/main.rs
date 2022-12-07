use std::{cell::RefCell, fmt::Debug, rc::Rc, str::FromStr};

const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

#[derive(Debug)]
enum Cmd {
    Cd(CdLocation),
    Ls(Vec<LsOutput>),
}

impl FromStr for Cmd {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(0);
        }

        let (command, rest) = s.split_at(2);
        match command {
            "cd" => {
                let directory = rest.trim();
                match directory {
                    "/" => Ok(Cmd::Cd(CdLocation::Root)),
                    ".." => Ok(Cmd::Cd(CdLocation::Out)),
                    _ => Ok(Cmd::Cd(CdLocation::In {
                        location: directory.to_string(),
                    })),
                }
            }
            "ls" => Ok(Self::Ls(
                rest.lines().skip(1).flat_map(|e| e.parse()).collect(),
            )),
            _ => Err(1),
        }
    }
}

#[derive(Debug)]
enum CdLocation {
    Root,
    In { location: String },
    Out,
}

#[derive(Debug)]
enum LsOutput {
    File { size: usize, filename: String },
    Dir { dirname: String },
}

impl FromStr for LsOutput {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").expect("Aoc is an angel");

        match left {
            "dir" => Ok(Self::Dir {
                dirname: right.to_string(),
            }),
            // here we have the size hopefully
            _ => Ok(Self::File {
                size: left.parse().expect("an angel"),
                filename: right.to_string(),
            }),
        }
    }
}

struct Folder {
    dirname: String,
    files: Vec<File>,
    parent: Option<Rc<RefCell<Folder>>>,
    folders: Vec<Rc<RefCell<Folder>>>,
}

struct File {
    _filename: String,
    size: usize,
}

fn explore_folder_for_q2(folder: &Folder) -> (usize, Vec<usize>) {
    let mut ret = vec![];
    let mut sub_size_sum = 0;

    for folder in &folder.folders {
        let (sub_size, all_sizes) = explore_folder_for_q2(&folder.borrow());
        sub_size_sum += sub_size;
        ret.extend(all_sizes);
    }
    let the_folder_size = folder.files.iter().map(|file| file.size).sum::<usize>() + sub_size_sum;

    ret.push(the_folder_size);

    (the_folder_size, ret)
}

fn explore_folder_for_answer_q1(folder: &Folder) -> (usize, usize) {
    let mut folder_size = 0;
    let mut folder_count = 0;
    for folder in &folder.folders {
        let (size, count) = explore_folder_for_answer_q1(&folder.borrow());
        folder_size += size;
        folder_count += count;
    }

    let the_folder_size = folder.files.iter().map(|file| file.size).sum::<usize>() + folder_size;

    if the_folder_size < 100_000 {
        folder_count += the_folder_size;
    }
    (the_folder_size, folder_count)
}

fn q1() {
    let history = include_str!("input.txt");

    let commands: Vec<Cmd> = history.split("$ ").flat_map(|bob| bob.parse()).collect();

    let mut current_folder = Rc::new(RefCell::new(Folder {
        dirname: "/".to_string(),
        files: vec![],
        parent: None,
        folders: vec![],
    }));

    for command in commands {
        match command {
            Cmd::Cd(cd) => match cd {
                CdLocation::Root => {
                    while let Some(parent) = &current_folder.clone().borrow().parent {
                        current_folder = parent.clone();
                    }
                }
                CdLocation::In { location } => {
                    match current_folder
                        .clone()
                        .borrow()
                        .folders
                        .iter()
                        .find(|folder| folder.borrow().dirname == location)
                    {
                        Some(folder) => current_folder = folder.clone(),
                        None => {}
                    }
                }
                CdLocation::Out => {
                    current_folder = current_folder
                        .clone()
                        .borrow()
                        .parent
                        .as_ref()
                        .expect("not supposed to be root")
                        .clone()
                }
            },
            Cmd::Ls(ls) => {
                for ls_output in ls {
                    match ls_output {
                        LsOutput::File { size, filename } => {
                            current_folder.borrow_mut().files.push(File {
                                _filename: filename,
                                size,
                            })
                        }
                        LsOutput::Dir { dirname } => {
                            current_folder
                                .borrow_mut()
                                .folders
                                .push(Rc::new(RefCell::new(Folder {
                                    dirname,
                                    files: vec![],
                                    parent: Some(current_folder.clone()),
                                    folders: vec![],
                                })))
                        }
                    }
                }
            }
        }
    }

    while let Some(parent) = &current_folder.clone().borrow().parent {
        current_folder = parent.clone();
    }

    let (_, res) = explore_folder_for_answer_q1(&current_folder.borrow());

    println!("q1 : {res:?}");

    let q2 = explore_folder_for_q2(&current_folder.borrow());

    let current_space = TOTAL_SPACE - q2.0;

    let answer =
        q2.1.iter()
            .filter(|&&space_taken| current_space + space_taken > NEEDED_SPACE)
            .min()
            .expect("surely");
    println!("q2 : {answer:?}");
}

fn main() {
    q1();
}
