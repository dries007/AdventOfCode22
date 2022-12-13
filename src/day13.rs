use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;
use std::iter::{Peekable, zip};
use std::str::Chars;

use itertools::Itertools;

use crate::day13::Data::{INT, LIST};
use crate::helpers::*;

const EXAMPLE_1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";


pub fn day13() -> Result<()> {
    println!("Day 13");
    let input = fs::read_to_string("input13.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 13);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 140);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Data {
    LIST(Vec<Data>),
    INT(u32),
}

impl PartialOrd<Self> for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // println!("Compare {:?} with {:?}", self, other);
        match (self, other) {
            (INT(left), INT(right)) => {
                return left.partial_cmp(right);
            }
            (LIST(left), LIST(right)) => {
                for (a, b) in zip(left, right) {
                    let cmp = a.partial_cmp(b).unwrap();
                    if cmp != Equal {
                        return Some(cmp);
                    }
                }
                return Some(if left.len() == right.len() {
                    Equal
                } else if left.len() < right.len() {
                    Less
                } else {
                    Greater
                })

            }
            (INT(left), right) => LIST(vec![INT(*left)]).partial_cmp(&right),
            (left, INT(right)) => left.partial_cmp(&LIST(vec![INT(*right)])),

        }
    }
}

impl Data {
    fn parse(line: &mut Peekable<Chars>) -> Self {
        match line.peek().unwrap() {
            '[' => {
                line.next(); // discard [
                let mut contents = vec![];
                loop {
                    // The only 2 control characters within the scope of this list are `]` and `,`.
                    match line.peek().unwrap() {
                        ']' => {
                            line.next(); // discard ]
                            break
                        }
                        ',' => {
                            line.next(); // discard ,
                        }
                        _ => { // In every other case,
                            contents.push(Data::parse(line));
                        }
                    }
                }
                LIST(contents)
            }
            _ => {
                let mut i = 0;
                while line.peek().unwrap().is_ascii_digit() {
                    i *= 10;
                    i += line.next().unwrap().to_digit(10).unwrap();
                }
                INT(i)
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let packets = input.lines().filter(|&line| !line.is_empty()).map(|line| Data::parse(&mut line.chars().peekable())).collect_vec();

    let score = packets.chunks_exact(2).enumerate().map(|(i, x)| if let [a, b] = x {
        // println!();
        return if a.partial_cmp(b).unwrap() == Less {
            i + 1
        } else {
            0
        }
    } else { panic!("unpossible") } ).sum();

    assert_ne!(score, 6182); // Too high

    return Ok(score);
}

fn part2(input: &str) -> Result<usize> {
    let input = input.to_string() + "\n[[2]]\n[[6]]";

    let mut packets = input.lines().filter(|&line| !line.is_empty()).map(|line| Data::parse(&mut line.chars().peekable())).collect_vec();
    packets.sort();

    // for x in packets {
    //     println!("{:?}", x);
    // }

    // elf indexing from 1
    let a = 1 + packets.iter().position(|x| x == &LIST(vec![LIST(vec![INT(2)])])).unwrap();
    let b = 1 + packets.iter().position(|x| x == &LIST(vec![LIST(vec![INT(6)])])).unwrap();

    return Ok(a * b);
}
