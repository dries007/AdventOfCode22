use std::collections::HashSet;
use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


pub fn day3() -> Result<()> {
    println!("Day 3");
    let input = fs::read_to_string("input3.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 157);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 70);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn part1(input: &str) -> Result<u32> {
    return Ok(input.split("\n").map( |line| {
        let (a, b) = line.split_at(line.len() / 2);
        let a_chars = HashSet::<_>::from_iter(a.chars());
        let b_chars = HashSet::<_>::from_iter(b.chars());
        let union = &a_chars & &b_chars;
        assert_eq!(union.len(), 1);
        let c = union.iter().nth(0).unwrap();
        let mut s = c.to_digit(36).unwrap() - 9;
        if c.is_ascii_uppercase() {
            s += 26;
        }
        return s;
    }).sum());
}

fn part2(input: &str) -> Result<u32> {
    let mut score = 0;
    for lines in input.split('\n').collect::<Vec<&str>>().chunks(3) {
        let a = HashSet::<_>::from_iter(lines[0].chars());
        let b = HashSet::<_>::from_iter(lines[1].chars());
        let c = HashSet::<_>::from_iter(lines[2].chars());
        let union = &(&a & &b) & &c;
        assert_eq!(union.len(), 1);
        let c = union.iter().nth(0).unwrap();
        let mut s = c.to_digit(36).unwrap() - 9;
        if c.is_ascii_uppercase() {
            s += 26;
        }
        score += s;
    }
    return Ok(score);
}
