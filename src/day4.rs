use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";


pub fn day4() -> Result<()> {
    println!("Day 4");
    let input = fs::read_to_string("input4.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 2);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 4);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn parse_pairs(input: &str) -> (i32, i32) {
    let x: Vec<i32> = input.split('-').map(|x| x.parse().unwrap()).collect();
    return (x[0], x[1])
}

fn part1(input: &str) -> Result<i32> {
    let mut score = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let (s1, e1) = parse_pairs(split[0]);
        let (s2, e2) = parse_pairs(split[1]);

        if (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1) {
            score += 1;
        }
    }
    return Ok(score);
}

fn part2(input: &str) -> Result<i32> {
    let mut score = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let (s1, e1) = parse_pairs(split[0]);
        let (s2, e2) = parse_pairs(split[1]);

        let r1 = s1..=e1;
        let r2 = s2..=e2;

        if r1.contains(&s2) || r1.contains(&e2) ||
            r2.contains(&s1) || r2.contains(&e1) {
            score += 1;
        }
    }
    return Ok(score);
}
