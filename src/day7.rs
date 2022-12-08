use std::collections::HashMap;
use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";


pub fn day7() -> Result<()> {
    println!("Day 7");
    let input = fs::read_to_string("input7.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 95437);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 0);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: usize,
}

impl Dir {
    fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: 0,
        }
    }
}


fn part1(input: &str) -> Result<i32> {
    let mut root = Dir::new();
    let mut current: &Dir = &mut root;

    let mut cd : Vec<String> = vec![];

    for line in input.lines() {
        match &line[..4] {
            "$ ls" => println!("ls of {:?}: {:?}", cd, current),
            "$ cd" => {
                println!("cd, before: {:?}", cd);
                if line == "$ cd /" {
                    current = &root;
                } else if line == "$ cd .." {
                    cd.pop();

                    current = &root;
                    for p in &cd {
                        current = current.dirs.get(p).unwrap();
                    }

                } else {
                    let dirname = line[5..].to_string();
                    cd.push(dirname.clone());
                    let mut new_dir = Dir::new();
                    current.dirs.insert(dirname, new_dir);
                    current = &new_dir;
                }
                println!("cd, after: {:?}", cd);
            },
            _ => println!("other: {}", line),
        }

    }


    return Ok(0);
}

fn part2(input: &str) -> Result<i32> {
    return Ok(0);
}
