use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut knots = [Position::default(); 10];

    let mut tail_positions = vec![Position::default()];

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
                    Direction::Up => knots[0].y += 1,
                    Direction::Down => knots[0].y -= 1,
                    Direction::Left => knots[0].x -= 1,
                    Direction::Right => knots[0].x += 1,
                }

                for i in 0..9 {
                    if knots[i].x == knots[i + 1].x && knots[i].y == knots[i + 1].y {
                        // No need to do anything
                    } else if (knots[i].x == knots[i + 1].x
                        && (knots[i].y - knots[i + 1].y).abs() == 1)
                        || (knots[i].y == knots[i + 1].y
                            && (knots[i].x - knots[i + 1].x).abs() == 1)
                        || ((knots[i].x - knots[i + 1].x).abs() == 1
                            && (knots[i].y - knots[i + 1].y).abs() == 1)
                    {
                        // No need to do anything
                    } else {
                        if knots[i].x == knots[i + 1].x {
                            if (knots[i].y - knots[i + 1].y).abs() == 2 {
                                knots[i + 1].y += (knots[i].y - knots[i + 1].y).sign()
                                    * ((knots[i].y - knots[i + 1].y).abs() - 1);
                            } else {
                                panic!();
                            }
                        } else if knots[i].y == knots[i + 1].y {
                            if (knots[i].x - knots[i + 1].x).abs() == 2 {
                                knots[i + 1].x += (knots[i].x - knots[i + 1].x).sign()
                                    * ((knots[i].x - knots[i + 1].x).abs() - 1);
                            } else {
                                panic!();
                            }
                        } else if ((knots[i].x - knots[i + 1].x).abs() == 2
                            && (knots[i].y - knots[i + 1].y).abs() == 1)
                            || ((knots[i].y - knots[i + 1].y).abs() == 2
                                && (knots[i].x - knots[i + 1].x).abs() == 1)
                            || ((knots[i].x - knots[i + 1].x).abs() == 2
                                && (knots[i].y - knots[i + 1].y).abs() == 2)
                        {
                            knots[i + 1].x += 1 * (knots[i].x - knots[i + 1].x).sign();
                            knots[i + 1].y += 1 * (knots[i].y - knots[i + 1].y).sign();
                        } else {
                            panic!();
                        }
                    }
                }

                tail_positions.push(knots[9].clone());
            }
        });

    println!("{}", tail_positions.into_iter().unique().count());
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    count: u32,
}

#[derive(Debug)]
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
