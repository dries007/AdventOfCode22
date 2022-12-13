use std::fs;
use ansi_term::Color::Cyan;
use ansi_term::Color::Green;
use itertools::{Itertools, min};

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
    // assert_eq!(part2(EXAMPLE_1)?, 0);
    // println!("\tPart 2: {}", part2(input.as_str())?);
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

fn print_field(field: &Vec<Vec<usize>>, path: &Vec<(usize, usize)>, end: (usize, usize)) {
    println!("Field:\n{}", field.iter().enumerate().map(|(y, line)| { line.iter().enumerate().map(|(x, &height)| {
        let c = char::from_u32(('a' as usize + height) as u32).unwrap();
        if (x, y) == end {
            Green.paint(format!("{}", c)).to_string()
        } else if path.contains(&(x, y)) {
            Cyan.paint(format!("{}", c)).to_string()
        } else {
            format!("{}", c)
        }
    }).join("") }).join("\n"));
}

fn step(field: &Vec<Vec<usize>>, path: Vec<(usize, usize)>, end: (usize, usize), cutoff: Option<&usize>) -> Option<usize> {
    print_field(&field, &path, end);

    let &pos = path.last().unwrap();
    if cutoff.is_some() && *cutoff.unwrap() <= path.len() {
        println!("Path {:?} ({}) longer than cutoff {}, stopping here.", path, path.len(), *cutoff.unwrap());
        return None
    }
    if pos == end {
        println!("Reached end in {} steps!", path.len());
        print_field(&field, &path, end);
        return Some(path.len())
    }

    let max = (field[0].len()-1, field.len()-1);
    let max_h = field[pos.1][pos.0] + 1;
    let u = if pos.1 != 0 && !path.contains(&(pos.0, pos.1-1)) {Some(field[pos.1-1][pos.0])} else {None};
    let d = if pos.1 != max.1 && !path.contains(&(pos.0, pos.1+1)) {Some(field[pos.1+1][pos.0])} else {None};
    let l = if pos.0 != 0 && !path.contains(&(pos.0-1, pos.1)) {Some(field[pos.1][pos.0-1])} else {None};
    let r = if pos.0 != max.0 && !path.contains(&(pos.0+1, pos.1)) {Some(field[pos.1][pos.0+1])} else {None};
    // println!("At {:?} ({}): u={:?} d={:?} l={:?} r={:?}", pos, max_h, u, d, l, r);

    let mut options = vec![];

    if cutoff.is_some() {
        options.push(*cutoff.unwrap());
    }

    if r.is_some_and(|&h| h <= max_h) {
        // println!("Step R");
        let mut new_path = path.clone();
        new_path.push((pos.0+1, pos.1));
        let r = step(field, new_path, end, min(&options));
        if r.is_some() {
            options.push(r.unwrap())
        }
    }
    if u.is_some_and(|&h| h <= max_h) {
        // println!("Step U");
        let mut new_path = path.clone();
        new_path.push((pos.0, pos.1-1));
        let r = step(field, new_path, end, min(&options));
        if r.is_some() {
            options.push(r.unwrap())
        }
    }
    if d.is_some_and(|&h| h <= max_h) {
        // println!("Step D");
        let mut new_path = path.clone();
        new_path.push((pos.0, pos.1+1));
        let r = step(field, new_path, end, min(&options));
        if r.is_some() {
            options.push(r.unwrap())
        }
    }
    if l.is_some_and(|&h| h <= max_h) {
        // println!("Step L");
        let mut new_path = path.clone();
        new_path.push((pos.0-1, pos.1));
        let r = step(field, new_path, end, min(&options));
        if r.is_some() {
            options.push(r.unwrap())
        }
    }

    return min(options);
}

fn part1(input: &str) -> Result<usize> {
    let (field, start, end) = parse(input);
    let count = step(&field, vec![start], end, None).unwrap();
    println!("Path len {}", count);
    return Ok(count-1);
}

fn part2(input: &str) -> Result<i32> {
    return Ok(0);
}
