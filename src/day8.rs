use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use ansi_term::Colour::Cyan;

use crate::helpers::*;

const EXAMPLE_1: &str = "30373
25512
65332
33549
35390";


pub fn day8() -> Result<()> {
    println!("Day 8");
    let input = fs::read_to_string("input8.txt")?;
    // assert_eq!(part1(EXAMPLE_1)?, 21);
    // println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 8);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    return input.lines().map(|line| {
        line.chars().map(|c| {
            c.to_string().parse::<u32>().unwrap()
        }).collect()
    }).collect()
}


fn print_field(field: &Vec<Vec<u32>>, visible: &HashSet<(usize, usize)>) {
    for y in 0..field.len() {
        let line = &field[y];
        for x in 0..line.len() {
            if visible.contains(&(x, y)) {
                print!("{}", Cyan.paint(format!("{}", line[x])));
            } else {
                print!("{}", line[x]);
            }
        }
        println!();
    }
}


fn part1(input: &str) -> Result<usize> {
    let field = parse(input);

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // Ray casting: X+
    for y in 0..field.len() {
        let line = &field[y];
        let mut prev = line[0];
        // First is always visible.
        visible.insert((0, y));
        for x in 1..line.len() {
            if prev < line[x] {
                visible.insert((x, y));
                prev = max(line[x], prev);
            }
        }
    }
    // Ray casting: X-
    for y in 0..field.len() {
        let line = &field[y];
        let start_x = line.len()-1;
        let mut prev = line[start_x];
        // First is always visible.
        visible.insert((start_x, y));
        for x in 1..line.len() {
            if prev < line[start_x-x] {
                visible.insert((start_x-x, y));
                prev = max(line[start_x-x], prev);
            }
        }
    }
    // Ray casting: Y+
    for x in 0..field.len() {
        let mut prev = field[0][x];
        // First is always visible.
        visible.insert((x, 0));
        for y in 1..field.len() {
            if prev < field[y][x] {
                visible.insert((x, y));
                prev = max(field[y][x], prev);
            }
        }
    }
    // Ray casting: Y-
    for x in 0..field.len() {
        let start_y = field.len()-1;
        let mut prev = field[start_y][x];
        // First is always visible.
        visible.insert((x, start_y));
        for y in 1..field.len() {
            if prev < field[start_y-y][x] {
                visible.insert((x, start_y-y));
                prev = max(field[start_y-y][x], prev);
            }
        }
    }

    print_field(&field, &visible);

    return Ok(visible.len());
}

fn scenic_score(x: usize, y: usize, field: &Vec<Vec<u32>>) -> u32 {
    let height = field[y][x];
    let mut scores = vec![0, 0, 0, 0];
    // Ray casting: X+
    for xx in x+1..field[y].len() {
        scores[0] += 1;
        if field[y][xx] >= height {
            break
        }
    }
    // Ray casting: X-
    for xx in 1..x+1 {
        scores[1] += 1;
        if field[y][x-xx] >= height {
            break
        }
    }
    // Ray casting: Y+
    for yy in y+1..field.len() {
        scores[2] += 1;
        if field[yy][x] >= height {
            break
        }
    }
    // Ray casting: Y-
    for yy in 1..y+1 {
        scores[3] += 1;
        if field[y-yy][x] >= height {
            break
        }
    }
    return scores[0] * scores[1] * scores[2] * scores[3];
}

fn part2(input: &str) -> Result<u32> {
    let field = parse(input);
    let mut max_score = 0;

    for y in 1..field.len() - 1 {
        for x in 1..field[y].len() - 1 {
            let score = scenic_score(x, y, &field);
            print!("{}", score);
            // println!("{},{} ({}) = {}", x, y, field[y][x], score);
            if score > max_score {
                max_score = score;
            }
        }
        println!()
    }

    return Ok(max_score);
}
