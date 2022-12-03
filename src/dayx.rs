use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "";


pub fn dayx() -> Result<()> {
    println!("Day X");
    let input = fs::read_to_string("inputx.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 0);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 0);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn part1(input: &str) -> Result<i32> {
    return Ok(0);
}

fn part2(input: &str) -> Result<i32> {
    return Ok(0);
}
