use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "A Y
B X
C Z
";


pub fn day2() -> Result<()> {
    println!("Day 2");
    let input = fs::read_to_string("input2.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 15);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 12);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug, PartialEq, Eq)]
enum Hands {
    ROCK,
    PAPER,
    SCISSORS,
}

fn char_to_hands(c: char) -> Hands {
    return match c {
        'A' => Hands::ROCK,
        'B' => Hands::PAPER,
        'C' => Hands::SCISSORS,
        'X' => Hands::ROCK,
        'Y' => Hands::PAPER,
        'Z' => Hands::SCISSORS,
        _ => panic!("Illegal input"),
    }
}

fn match_score(opponent: Hands, you: Hands) -> i32 {
    if opponent == you {
        return 3
    }
    match (opponent, you) {
        (Hands::ROCK, Hands::PAPER) => 6,
        (Hands::PAPER, Hands::SCISSORS) => 6,
        (Hands::SCISSORS, Hands::ROCK) => 6,
        _ => 0,
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut score = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue
        }

        let o = line.chars().nth(0).map(char_to_hands).unwrap();
        let r = line.chars().nth(2).map(char_to_hands).unwrap();

        score += match r {
            Hands::ROCK => 1,
            Hands::PAPER => 2,
            Hands::SCISSORS => 3,
        };

        score += match_score(o, r);
    }
    return Ok(score);
}

fn part2(input: &str) -> Result<i32> {
    let mut score = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue
        }

        let o = line.chars().nth(0).map(char_to_hands).unwrap();
        let r = line.chars().nth(2).unwrap();

        let play = match r {
            // Loss
            'X' => match o {
                Hands::ROCK => Hands::SCISSORS,
                Hands::PAPER => Hands::ROCK,
                Hands::SCISSORS => Hands::PAPER,
            }
            // Draw
            'Y' => o,
            // Win
            'Z' => match o {
                Hands::ROCK => Hands::PAPER,
                Hands::PAPER => Hands::SCISSORS,
                Hands::SCISSORS => Hands::ROCK,
            },
            _ => panic!("Illegal input"),
        };
        score += match play {
            Hands::ROCK => 1,
            Hands::PAPER => 2,
            Hands::SCISSORS => 3,
        };
        score += match r {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Illegal input"),
        };
    }
    return Ok(score);
}

