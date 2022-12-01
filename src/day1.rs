use std::fs;

use crate::helpers::*;

const EXAMPLE_1: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";


pub fn day1() -> Result<()> {
    println!("Day 1");
    let input = fs::read_to_string("input1.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 24000);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 45000);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}


fn part1(input: &str) -> Result<i32> {
    let mut largest_sum = 0;
    let mut sum = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            if sum > largest_sum {
                largest_sum = sum;
            }
            sum = 0;
            continue;
        }
        let i = line.parse::<i32>()?;
        sum += i;
    }
    if sum > largest_sum {
        largest_sum = sum;
    }

    return Ok(largest_sum);
}


fn part2(input: &str) -> Result<i32> {
    let mut sums = Vec::new();
    let mut sum = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }
        let i = line.parse::<i32>()?;
        sum += i;
    }
    sums.push(sum);
    sums.sort();
    return Ok(sums[sums.len()-1] + sums[sums.len()-2] + sums[sums.len()-3]);
}

