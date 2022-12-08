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

    let mut visibility = vec![vec![false; columns]; rows];

    for row_index in 0..rows {
        for column_index in 0..columns {
            if row_index == 0 || row_index == (rows - 1) {
                visibility[row_index][column_index] = true;
            } else if column_index == 0 || column_index == (columns - 1) {
                visibility[row_index][column_index] = true;
            }
        }
    }

    // Up
    let mut tallest_tree = 0;

    for column_index in 0..columns {
        for row_index in 0..rows {
            if forest[row_index][column_index] > tallest_tree {
                tallest_tree = forest[row_index][column_index];
                visibility[row_index][column_index] = true;
            }
        }

        tallest_tree = 0;
    }

    // Down
    let mut tallest_tree = 0;

    for column_index in 0..columns {
        for row_index in (0..rows).into_iter().rev() {
            if forest[row_index][column_index] > tallest_tree {
                tallest_tree = forest[row_index][column_index];
                visibility[row_index][column_index] = true;
            }
        }

        tallest_tree = 0;
    }

    // Left
    for row_index in 0..rows {
        let mut tallest_tree = 0;

        for column_index in 0..columns {
            if forest[row_index][column_index] > tallest_tree {
                tallest_tree = forest[row_index][column_index];
                visibility[row_index][column_index] = true;
            }
        }
    }

    // Right
    for row_index in 0..rows {
        let mut tallest_tree = 0;

        for column_index in (0..columns).into_iter().rev() {
            if forest[row_index][column_index] > tallest_tree {
                tallest_tree = forest[row_index][column_index];
                visibility[row_index][column_index] = true;
            }
        }
    }

    let visible_trees = visibility
        .into_iter()
        .flatten()
        .filter(|is_visible| *is_visible)
        .count();

    println!("{visible_trees}");
}
