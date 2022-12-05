use std::fs;
use std::iter::zip;

use itertools::Itertools;

use crate::helpers::*;

const EXAMPLE_1: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";


pub fn day5() -> Result<()> {
    println!("Day 5");
    let input = fs::read_to_string("input5.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, "CMZ");
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, "");
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut i = input.lines().rev();
    let digits = i.next().unwrap().replace(" ", "").len();
    let mut state: Vec<Vec<char>> = vec![vec![]; digits];
    for line in i {
        let chunks= line.chars().chunks(4);
        for (i, mut chars) in zip(0..digits, chunks.into_iter()) {
            let c = chars.nth(1).unwrap();
            if c != ' ' {
                state[i].push(c);
                println!("{:?} - {:?}", i, state[i]);

            }
        }
    }
    return state;
}

fn print_state(state: Vec<Vec<char>> ) {
    let maxlen = state.iter().map(Vec::len).max().unwrap();
    println!("State {}: {:?}", maxlen, state);
    for h in 1..=maxlen {
        for tower in state.iter() {
            print!("{}", tower.get(maxlen - h).unwrap_or(&' '));
        }
        println!();
    }
    for i in 1..=state.len() {
        print!("{}", i);
    }
    println!();
}

fn part1(input: &str) -> Result<&str> {

    let split = input.split("\n\n").collect::<Vec<&str>>();
    let mut state = parse(split[0]);

    print_state(state);

    return Ok("");
}

fn part2(input: &str) -> Result<&str> {
    return Ok("");
}
