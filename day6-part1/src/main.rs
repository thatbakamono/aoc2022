use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut n = 3;

    input
        .trim()
        .chars()
        .tuple_windows()
        .find(|(first, second, third, fourth)| {
            n += 1;

            [first, second, third, fourth].into_iter().unique().count() == 4
        });

    println!("{n}");
}
