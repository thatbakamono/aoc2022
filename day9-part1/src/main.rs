use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut head = Position::default();
    let mut tail = Position::default();

    let mut tail_positions = vec![tail.clone()];

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();

            let direction = match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };

            let count = parts[1].parse::<u32>().unwrap();

            Movement { direction, count }
        })
        .for_each(|movement| {
            for _ in 0..movement.count {
                match movement.direction {
                    Direction::Up => head.y += 1,
                    Direction::Down => head.y -= 1,
                    Direction::Left => head.x -= 1,
                    Direction::Right => head.x += 1,
                }

                if head.x == tail.x && head.y == tail.y {
                    // Head is on top of the tail, no need to do anything
                } else if (head.x == tail.x && (head.y - tail.y).abs() == 1)
                    || (head.y == tail.y && (head.x - tail.x).abs() == 1)
                    || ((head.x - tail.x).abs() == 1 && (head.y - tail.y).abs() == 1)
                {
                    // Tail is touching the head vertically, horizontally or diagonally, no need to do anything
                } else {
                    if head.x == tail.x {
                        if (head.y - tail.y).abs() == 2 {
                            tail.y += (head.y - tail.y).sign() * ((head.y - tail.y).abs() - 1);
                        } else {
                            panic!("");
                        }
                    } else if head.y == tail.y {
                        if (head.x - tail.x).abs() == 2 {
                            tail.x += (head.x - tail.x).sign() * ((head.x - tail.x).abs() - 1);
                        } else {
                            panic!("");
                        }
                    } else if ((head.x - tail.x).abs() == 2 && (head.y - tail.y).abs() == 1)
                        || ((head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 1)
                    {
                        tail.x += 1 * (head.x - tail.x).sign();
                        tail.y += 1 * (head.y - tail.y).sign();
                    } else {
                        panic!("");
                    }
                }

                tail_positions.push(tail.clone());
            }
        });

    println!("{}", tail_positions.into_iter().unique().count());
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Movement {
    direction: Direction,
    count: u32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait NumberExt {
    fn sign(&self) -> Self;
}

impl NumberExt for i32 {
    fn sign(&self) -> Self {
        if *self >= 0 {
            1
        } else {
            -1
        }
    }
}
