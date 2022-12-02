use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let score = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let opponents_move = match line.chars().nth(0).unwrap() {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => unreachable!(),
            };

            let outcome = match line.chars().nth(2).unwrap() {
                'X' => Outcome::Loss,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => unreachable!(),
            };

            let your_response = match opponents_move {
                Shape::Rock => match outcome {
                    Outcome::Win => Shape::Paper,
                    Outcome::Draw => Shape::Rock,
                    Outcome::Loss => Shape::Scissors,
                },
                Shape::Paper => match outcome {
                    Outcome::Win => Shape::Scissors,
                    Outcome::Draw => Shape::Paper,
                    Outcome::Loss => Shape::Rock,
                },
                Shape::Scissors => match outcome {
                    Outcome::Win => Shape::Rock,
                    Outcome::Draw => Shape::Scissors,
                    Outcome::Loss => Shape::Paper,
                },
            };

            let score = match outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            } + match your_response {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };

            score
        })
        .sum::<usize>();

    println!("{score}");
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}
