use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut n = 13;

    input
        .trim()
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .find(|slice| {
            n += 1;

            slice.iter().unique().count() == 14
        });

    println!("{n}");
}
