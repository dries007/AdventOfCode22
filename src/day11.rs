use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use crate::day11::Operation::{AddConst, MultiplyConst, MultiplyOld};

use crate::helpers::*;

const EXAMPLE_1: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";


pub fn day11() -> Result<()> {
    println!("Day 11");
    let input = fs::read_to_string("input11.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 10605);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 2713310158);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    AddConst(u64),
    MultiplyConst(u64),
    MultiplyOld(),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "* old" {
            Ok(MultiplyOld())
        } else if s.starts_with("+ ") {
            Ok(AddConst(s.strip_prefix("+ ").unwrap().parse().unwrap()))
        } else if s.starts_with("* ") {
            Ok(MultiplyConst(s.strip_prefix("* ").unwrap().parse().unwrap()))
        } else {
            Err("Shit's fucked yo.".to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    nr: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    count: u64,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let nr = lines.next().unwrap().strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap().parse().unwrap();
        let items = lines.next().unwrap().strip_prefix("  Starting items: ").unwrap().split(", ").map(|x| x.parse().unwrap()).collect_vec();
        let operation = lines.next().unwrap().strip_prefix("  Operation: new = old ").unwrap().parse().unwrap();
        let test = lines.next().unwrap().strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
        let if_true = lines.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
        let if_false = lines.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
        return Monkey{nr, items, operation, test, if_true, if_false, count: 0}
    }
}

fn part1(input: &str) -> Result<u64> {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect_vec();
    let mut mailbox: Vec<Vec<u64>> = vec![vec![]; monkeys.len()]; // Le sigh

    for round in 0..20 {
        // println!("START Round {}", round);
        for monkey in &mut monkeys {
            monkey.items.append(&mut mailbox[monkey.nr]);
            // println!("M{}: {:?}", monkey.nr, monkey.items);

            for item in monkey.items.drain(0..) {
                monkey.count += 1;
                let new_item = match monkey.operation {
                    AddConst(x) => item + x,
                    MultiplyConst(x) => item * x,
                    MultiplyOld() => item * item,
                } / 3;
                let to = if new_item % monkey.test == 0 {monkey.if_true} else {monkey.if_false};
                // println!("Monkey {} throws item {} to monkey {} with new value {}", monkey.nr, item, to, new_item);
                mailbox[to].push(new_item);
                // monkeys[to].items.push(new_item);
            }
        }

        // println!("END Round {}", round);
        for monkey in &mut monkeys {
            monkey.items.append(&mut mailbox[monkey.nr]);
            // println!("M{}: {:?}", monkey.nr, monkey.items);
        }
    }

    let leaderboard = monkeys.iter().sorted_by_key(|x| x.count).rev().collect_vec();
    // for x in leaderboard.iter() {
    //     println!("M{} did {} items", x.nr, x.count);
    // }

    return Ok(leaderboard[0].count * leaderboard[1].count);
}

fn part2(input: &str) -> Result<u64> {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect_vec();
    let mut mailbox: Vec<Vec<u64>> = vec![vec![]; monkeys.len()]; // Le sigh

    // We only need to keep track of the modulo of the fear factor, not the actual nr.
    let modulo = monkeys.iter().map(|x| x.test).reduce(|acc, e| acc * e).unwrap();

    for round in 0..10000 {
        // println!("START Round {}", round);
        for monkey in &mut monkeys {
            monkey.items.append(&mut mailbox[monkey.nr]);
            // println!("M{}: {:?}", monkey.nr, monkey.items);

            for item in monkey.items.drain(0..) {
                monkey.count += 1;
                let new_item = match monkey.operation {
                    AddConst(x) => item + x,
                    MultiplyConst(x) => item * x,
                    MultiplyOld() => item * item,
                } % modulo;
                let to = if new_item % monkey.test == 0 {monkey.if_true} else {monkey.if_false};
                // println!("Monkey {} throws item {} to monkey {} with new value {}", monkey.nr, item, to, new_item);
                mailbox[to].push(new_item);
                // monkeys[to].items.push(new_item);
            }
        }

        // println!("END Round {}", round);
        for monkey in &mut monkeys {
            monkey.items.append(&mut mailbox[monkey.nr]);
            // println!("M{}: {}", monkey.nr, monkey.count);
        }
    }

    let leaderboard = monkeys.iter().sorted_by_key(|x| x.count).rev().collect_vec();
    // for x in leaderboard.iter() {
    //     println!("M{} did {} items", x.nr, x.count);
    // }

    return Ok(leaderboard[0].count * leaderboard[1].count);
}
