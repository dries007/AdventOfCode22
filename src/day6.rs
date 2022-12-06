use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";


pub fn day6() -> Result<()> {
    println!("Day 6");
    let input = fs::read_to_string("input6.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

fn part1(input: &str) -> Result<usize> {
    for i in 4..input.len() {
        let prefix = &input[i-4..i].chars().collect::<Vec<char>>();

        if prefix[1..].contains(&prefix[0]) ||
            prefix[2..].contains(&prefix[1]) ||
            prefix[3..].contains(&prefix[2]) {
            continue
        }

        return Ok(i);
    }

    return Err("No pattern match".into());
}

fn contains_duplicates(prefix: &Vec<char>) -> bool {
    for j in 0..14 {
        if prefix[j+1..].contains(&prefix[j]) {
            return true
        }
    }
    return false
}

fn part2(input: &str) -> Result<usize> {

    for i in 14..input.len() {
        let prefix = &input[i-14..i].chars().collect::<Vec<char>>();
        if !contains_duplicates(prefix) {
            return Ok(i);
        }
    }

    return Err("No pattern match".into());
}
