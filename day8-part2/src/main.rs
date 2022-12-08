use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let forest = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|character| format!("{character}").parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = forest.len();
    let columns = forest[0].len();

    let mut scenic_scores = vec![vec![0; columns]; rows];

    for row_index in 0..rows {
        for column_index in 0..columns {
            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;

            // Up
            if row_index > 0 {
                for i in (0..row_index).into_iter().rev() {
                    up += 1;

                    if forest[row_index][column_index] <= forest[i][column_index] {
                        break;
                    }
                }
            }

            // Down
            if row_index < (rows - 1) {
                for i in (row_index + 1..rows).into_iter() {
                    down += 1;

                    if forest[row_index][column_index] <= forest[i][column_index] {
                        break;
                    }
                }
            }

            // Left
            if column_index > 0 {
                for i in (0..column_index).into_iter().rev() {
                    left += 1;

                    if forest[row_index][column_index] <= forest[row_index][i] {
                        break;
                    }
                }
            }

            // Right
            if column_index < (columns - 1) {
                for i in (column_index + 1..columns).into_iter() {
                    right += 1;

                    if forest[row_index][column_index] <= forest[row_index][i] {
                        break;
                    }
                }
            }

            scenic_scores[row_index][column_index] = up * down * left * right;
        }
    }

    let max_scenic_score = scenic_scores.into_iter().flatten().max().unwrap();

    println!("{max_scenic_score}");
}
