use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

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
    assert_eq!(part2(EXAMPLE_1)?, 24933642);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: usize,
    total: usize,
}

impl Dir {
    fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: 0,
            total: 0,
        }
    }
}

/// This builds a file system tree from the puzzel input, with total sizes calculated.
fn parse(input: &str) -> Dir {
    let mut root: Dir = Dir::new();
    let mut cd : Vec<String> = vec![];

    for line in input.lines() {
        match &line[..4] {
            "$ ls" => {
                // println!("ls of /{}", cd.join("/"))
            },
            "$ cd" => {
                // println!("cd, before: {:?}", cd);
                if line == "$ cd /" {
                    cd.clear();
                } else if line == "$ cd .." {
                    cd.pop();
                } else {
                    let dirname = line[5..].to_string();
                    cd.push(dirname.clone());
                }
            },
            _ => {
                // println!("other: {}", line);
                let mut dir = &mut root;
                for p in &cd {
                    dir = dir.dirs.get_mut(p).unwrap();
                }
                let (a, b): (&str, &str) = line.split(' ').collect_tuple().unwrap();
                if a == "dir" {
                    // println!("new dir: {}", b);
                    dir.dirs.insert(b.to_string(), Dir::new());
                } else {
                    // println!("file: {} = {}", b, a.parse::<usize>().unwrap());
                    dir.files += a.parse::<usize>().unwrap()
                }
            }
        }
    }

    fn calc_size(us: &mut Dir) -> usize {
        let children: usize = us.dirs.iter_mut().map(|(_name, x)| calc_size(x)).sum();
        us.total = us.files + children;
        return us.total;
    }
    calc_size(&mut root);

    return root;
}

fn part1(input: &str) -> Result<usize> {
    let root = parse(input);

    fn tree(name: &str, node: Dir, depth: usize) -> usize {
        // println!("{:depth$}- {} total={}", ' ', name, node.total);
        let mut acc = 0;
        if node.total < 100000 {
            acc += node.total;
        }
        for (name, node) in node.dirs {
            acc += tree(&name, node, depth + 1);
        }
        return acc;
    }

    let total= tree("/", root, 1);
    // println!("----");

    return Ok(total);
}

fn part2(input: &str) -> Result<usize> {
    let root = parse(input);

    fn unpack(node: &Dir) -> Vec<&Dir> {
        let mut nodes = vec![node];
        for (_name, node) in &node.dirs {
            nodes.append(&mut unpack(node));
        }
        return nodes;
    }

    let target = 30000000 - (70000000 - root.total);
    // println!("Target: {}", target);

    let mut nodes = unpack(&root);
    nodes.sort_by_key(|dir| dir.total);

    for x in nodes {
        if x.total < target {
            continue
        }
        return Ok(x.total);
    }

    panic!("unpossible");
}
