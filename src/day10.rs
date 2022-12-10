use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "noop
addx 3
addx -5";

const EXAMPLE_2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";


pub fn day10() -> Result<()> {
    println!("Day 10");
    let input = fs::read_to_string("input10.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 0);
    assert_eq!(part1(EXAMPLE_2)?, 13140);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_2)?, 0);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn part1(input: &str) -> Result<i32> {
    let mut cycles: i32 = 0;
    let mut reg_x: i32 = 1;
    let mut score: i32 = 0;

    let mut cycle = |x: i32| {
        cycles += 1;
        if (cycles - 20) % 40 == 0 {
            println!("X={} @ {}, score = {}", reg_x, cycles, score);
            score += reg_x * cycles;
        }
        reg_x += x;
    };

    for line in input.lines() {
        let mut split = line.split(' ');
        match split.next().unwrap() {
            "noop" => cycle(0),
            "addx" => {
                cycle(0);
                cycle(split.next().unwrap().parse::<i32>().unwrap());
            },
            _ => panic!("Shit's fucked yo."),
        }
    }

    return Ok(score);
}

fn part2(input: &str) -> Result<i32> {
    let mut cycles: i32 = 0;
    let mut reg_x: i32 = 1;

    let mut cycle = |x: i32| {
        let xmod = reg_x % 40;
        let cmod = cycles % 40;
        if cmod == xmod - 1 || cmod == xmod || cmod == xmod + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if cmod == 39 {
            println!();
        }
        cycles += 1;
        reg_x += x;
    };

    println!();
    for line in input.lines() {
        let mut split = line.split(' ');
        match split.next().unwrap() {
            "noop" => cycle(0),
            "addx" => {
                cycle(0);
                cycle(split.next().unwrap().parse::<i32>().unwrap());
            },
            _ => panic!("Shit's fucked yo."),
        }
    }
    println!();

    return Ok(0);
}
