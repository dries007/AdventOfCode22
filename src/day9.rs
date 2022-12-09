use std::collections::HashSet;
use std::fs;

use ansi_term::Color::Red;

use crate::helpers::*;

const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";


pub fn day9() -> Result<()> {
    println!("Day 9");
    let input = fs::read_to_string("input9.txt")?;
    assert_eq!(part1(EXAMPLE_1)?, 13);
    println!("\tPart 1: {}", part1(input.as_str())?);
    assert_eq!(part2(EXAMPLE_1)?, 1);
    assert_eq!(part2(EXAMPLE_2)?, 36);
    println!("\tPart 2: {}", part2(input.as_str())?);
    return Ok(());
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_field(visited: &HashSet<Point>, head: Point, tail: Point) {
    let min_x = visited.iter().map(|e| e.x).min().unwrap() - 2;
    let min_y = visited.iter().map(|e| e.y).min().unwrap() - 2;
    let max_x = visited.iter().map(|e| e.x).max().unwrap() + 2;
    let max_y = visited.iter().map(|e| e.y).max().unwrap() + 2;

    print!("       ");
    for x in min_x..=max_x {
        if x < 0 {
            print!("{}", Red.paint(format!("{:1}", x.abs() % 10)));
        } else {
            print!("{:1}", x % 10)
        }
    }
    println!();
    for y in (min_y..=max_y).rev() {
        print!("{:4} | ", y);
        for x in min_x..=max_x {
            let p = Point{x, y};
            if p == head {
                print!("H");
            } else if p == tail {
                print!("T");
            } else if visited.contains(&p) {
                print!("#");
            } else if (x, y) == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut visited = HashSet::<Point>::new();

    let mut head = Point{x: 0, y: 0};
    let mut tail = Point{x: 0, y: 0};
    visited.insert(tail);

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let nr = parts.next().unwrap().parse::<usize>().unwrap();
        for step in 0..nr {
            // println!("Doing step {} of {} {}", step, direction, nr);

            head = match direction {
                "R" => Point{x: head.x + 1, y: head.y},
                "L" => Point{x: head.x - 1, y: head.y},
                "D" => Point{x: head.x, y: head.y - 1},
                "U" => Point{x: head.x, y: head.y + 1},
                _ => panic!("Shit's fucked yo"),
            };

            let dx = head.x - tail.x;
            let dy = head.y - tail.y;
            // println!("H:{:?} T:{:?} d: {:?}", head, tail, (dx, dy));

            if dx.abs() > 1 || dy.abs() > 1 {
                let x = if dx == 0 {0} else if dx < 0 {-1} else {1};
                let y = if dy == 0 {0} else if dy < 0 {-1} else {1};
                // println!("Move required {}, {}", x, y);
                tail = Point{x: tail.x + x, y: tail.y + y};

                visited.insert(tail);
            }

        }

        // print_field(&visited, head, tail);
    }

    return Ok(visited.len());
}


fn print_field2(visited: &HashSet<Point>, rope: &Vec<Point>) {
    let min_x = visited.iter().map(|e| e.x).min().unwrap() - 5;
    let min_y = visited.iter().map(|e| e.y).min().unwrap() - 5;
    let max_x = visited.iter().map(|e| e.x).max().unwrap() + 5;
    let max_y = visited.iter().map(|e| e.y).max().unwrap() + 5;

    print!("       ");
    for x in min_x..=max_x {
        if x < 0 {
            print!("{}", Red.paint(format!("{:1}", x.abs() % 10)));
        } else {
            print!("{:1}", x % 10)
        }
    }
    println!();
    for y in (min_y..=max_y).rev() {
        print!("{:4} | ", y);
        for x in min_x..=max_x {
            let p = Point{x, y};
            print!("{}", if rope.contains(&p) {
                rope.iter().position(|&e| e == p).unwrap().to_string()
            } else if visited.contains(&p) {
                "#".to_string()
            } else if p.x == 0 && p.y == 0 {
                "s".to_string()
            } else {
                ".".to_string()
            });
        }
        println!();
    }
}

fn part2(input: &str) -> Result<usize> {
    let mut visited = HashSet::<Point>::new();

    let mut rope = vec![Point{x: 0, y: 0}; 10];

    visited.insert(rope[9]);

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let nr = parts.next().unwrap().parse::<usize>().unwrap();
        for step in 0..nr {
            println!("Doing step {} of {} {}", step, direction, nr);

            rope[0] = match direction {
                "R" => Point{x: rope[0].x + 1, y: rope[0].y},
                "L" => Point{x: rope[0].x - 1, y: rope[0].y},
                "D" => Point{x: rope[0].x, y: rope[0].y - 1},
                "U" => Point{x: rope[0].x, y: rope[0].y + 1},
                _ => panic!("Shit's fucked yo"),
            };
            println!("Rope: {:?}", rope);

            for i in 1..rope.len() {
                let dx = rope[i-1].x - rope[i].x;
                let dy = rope[i-1].y - rope[i].y;
                if dx.abs() > 1 || dy.abs() > 1 {
                    let x = if dx == 0 {0} else if dx < 0 {-1} else {1};
                    let y = if dy == 0 {0} else if dy < 0 {-1} else {1};
                    // println!("Move required {}, {}", x, y);
                    rope[i] = Point{x: rope[i].x + x, y: rope[i].y + y};
                }
            }
            visited.insert(rope[9]);

            // print_field2(&visited, &rope);
        }

        print_field2(&visited, &rope);
    }

    return Ok(visited.len());
}
