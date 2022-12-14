use std::fs;
use itertools::Itertools;
use crate::day14::Tile::*;

use crate::helpers::*;

const EXAMPLE_1: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";


pub fn day14() -> Result<()> {
    println!("Day 14");
    assert_eq!(part1(EXAMPLE_1)?, 24);
    let input = fs::read_to_string("input14.txt")?;
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 93);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Air,
    Sand,
    Rock,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    let mut lines = vec![];
    let mut max_y = usize::MIN;

    for line in input.lines() {
        let line = line.split(" -> ").map(|pair|
            pair.split(',').map(|e| e.parse().unwrap()).collect_tuple::<(usize,usize)>().unwrap()
        ).collect_vec();
        max_y = max_y.max(line.iter().map(|&(_x, y)| y).max().unwrap());
        lines.push(line);
    }

    // #define infinity 1000
    let mut field = vec![vec![Air; 1000]; 3 + max_y];
    println!("H={} by W={}", field.len(), field[0].len());
    print_field(&field);

    for line in lines {
        let mut line = line.iter();
        let mut start = line.next().unwrap();
        for end in line {
            if start.0 == end.0 {
                // x is fixed, y moves
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    field[y][start.0] = Rock;
                }
            } else if start.1 == end.1 {
                for x in start.0.min(end.0)..=start.0.max(end.0) {
                    field[start.1][x] = Rock;
                }
            } else {
                panic!("Not allowed.");
            }
            start = end;
        }
    }

    return field;
}

fn print_field(field: &Vec<Vec<Tile>>) {
    for (y, line) in field.iter().enumerate() {
        let line = line.iter().enumerate().map(|(x, tile)| {
            if (500, 0) == (x, y) {
                return '+'
            }
            match tile {
                Air => '.',
                Sand => '0',
                Rock => '#',
            }
        }).join("");
        println!("{}", line);
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut field = parse(input);
    let mut score = 0;
    print_field(&field);
    let void_y = field.len();
    let void_x = field[0].len();
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            y += 1;
            if y >= void_y {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x] == Air {
                continue
            }
            if x == 0 {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x-1] == Air {
                x -= 1;
                continue
            }
            if x >= void_x {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x+1] == Air {
                x += 1;
                continue
            }
            score += 1;
            field[y-1][x] = Sand;
            // print_field(&field, source);
            break
        }
    }
}

fn part2(input: &str) -> Result<i32> {
    let mut field = parse(input);

    let floor = field.len()-1;
    for x in 0..field[0].len() {
        field[floor][x] = Rock;
    }

    let mut score = 0;
    print_field(&field);
    let void_y = field.len();
    let void_x = field[0].len();
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            y += 1;
            if y >= void_y {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x] == Air {
                continue
            }
            if x == 0 {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x-1] == Air {
                x -= 1;
                continue
            }
            if x >= void_x {
                print_field(&field);
                println!("Void {}", score);
                return Ok(score);
            }
            if field[y][x+1] == Air {
                x += 1;
                continue
            }
            score += 1;
            if (x, y-1) == (500, 0) {
                print_field(&field);
                println!("Full {}", score);
                return Ok(score);
            }
            field[y-1][x] = Sand;
            // print_field(&field, source);
            break
        }
    }
}
