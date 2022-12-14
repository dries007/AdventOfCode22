use std::{fs, usize};

use ansi_term::Color::{Cyan, Red};
use ansi_term::Color::Green;
use itertools::{Itertools, min};
use pathfinding::prelude::astar;

use crate::helpers::*;

const EXAMPLE_1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";


pub fn day12() -> Result<()> {
    println!("Day 12");
    let input = fs::read_to_string("input12.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 31);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 29);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn parse(input: &str) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let field = input.lines().enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, char)| {
        match char {
            'S' => {
                start = (x, y);
                0
            },
            'E' => {
                end = (x, y);
                25
            },
            _ => char as usize - 'a' as usize,
        }
    }).collect_vec()).collect_vec();
    return (field, start, end)
}

fn print_field(field: &Vec<Vec<usize>>, path: &Vec<(usize, usize)>, end: (usize, usize), start: (usize, usize)) {
    println!("Field:\n{}", field.iter().enumerate().map(|(y, line)| { line.iter().enumerate().map(|(x, &height)| {
        let c = char::from_u32(('a' as usize + height) as u32).unwrap();
        if (x, y) == end {
            Green.paint(format!("{}", c)).to_string()
        } else if (x, y) == start {
            Red.paint(format!("{}", c)).to_string()
        } else if path.contains(&(x, y)) {
            Cyan.paint(format!("{}", c)).to_string()
        } else {
            format!("{}", c)
        }
    }).join("") }).join("\n"));
}

fn find_path(field: &Vec<Vec<usize>>, start: &(usize, usize), end: &(usize, usize)) -> Option<(Vec<(usize, usize)>, usize)> {
    return astar(&start, |&(x, y)| {
        let max = (field[0].len()-1, field.len()-1);
        let max_h = field[y][x] + 1;
        let mut options = vec![];
        if x != 0 && field[y][x-1] <= max_h {
            options.push(((x-1, y), 5))
        }
        if y != 0 && field[y-1][x] <= max_h {
            options.push(((x, y-1), 5))
        }
        if x != max.0 && field[y][x+1] <= max_h {
            options.push(((x+1, y), 5))
        }
        if y != max.1 && field[y+1][x] <= max_h {
            options.push(((x, y+1), 5))
        }
        return options;

    }, |&(x, y)| (end.0.abs_diff(x) + end.1.abs_diff(y)) / 3, |&p| p == *end);
}

fn part1(input: &str) -> Result<usize> {
    let (field, start, end) = parse(input);
    // let count = step(&field, vec![start], end, None).unwrap();
    // println!("Path len {}", count);

    let result = find_path(&field, &start, &end);

    let (path, len) = result.unwrap();

    // println!("A*: {:?}: {}", path, path.len());

    // print_field(&field, &path, end, start);

    assert_ne!(len, 383);
    assert_ne!(len, 384);

    return Ok(path.len()-1);
}

fn part2(input: &str) -> Result<usize> {

    let (field, _start, end) = parse(input);

    // Hmm delicious brute forcing
    let mut score = usize::MAX;
    for (y, line) in field.iter().enumerate() {
        for (x, &h) in line.iter().enumerate() {
            if h != 0 {
                continue
            }
            if let Some((path, _)) = find_path(&field, &(x, y), &end) {
                let result = path.len() - 1;
                // println!("found possible path, starting from {},{} with len {}", x, y, result);
                // print_field(&field, &path, end, (x, y));
                if result < score {
                    score = result;
                }
            }
        }
    }

    return Ok(score);
}
