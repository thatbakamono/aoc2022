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

            let your_response = match line.chars().nth(2).unwrap() {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissors,
                _ => unreachable!(),
            };

            let outcome = match opponents_move {
                Shape::Rock => match your_response {
                    Shape::Rock => Outcome::Draw,
                    Shape::Paper => Outcome::Win,
                    Shape::Scissors => Outcome::Loss,
                },
                Shape::Paper => match your_response {
                    Shape::Rock => Outcome::Loss,
                    Shape::Paper => Outcome::Draw,
                    Shape::Scissors => Outcome::Win,
                },
                Shape::Scissors => match your_response {
                    Shape::Rock => Outcome::Win,
                    Shape::Paper => Outcome::Loss,
                    Shape::Scissors => Outcome::Draw,
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
