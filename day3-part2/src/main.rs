use itertools::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const LOWER_CASE_A_CODE: u32 = 'a' as u32;
const UPPER_CASE_A_CODE: u32 = 'A' as u32;

fn main() {
    let sum = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first_compartment: HashSet<char> =
                HashSet::from_iter(chunk.next().unwrap().chars());
            let second_compartment: HashSet<char> =
                HashSet::from_iter(chunk.next().unwrap().chars());

            chunk
                .next()
                .unwrap()
                .chars()
                .filter(|item| {
                    first_compartment.contains(item) && second_compartment.contains(item)
                })
                .map(|item| match item {
                    'a'..='z' => item as u32 - LOWER_CASE_A_CODE + 1,
                    'A'..='Z' => item as u32 - UPPER_CASE_A_CODE + 27,
                    _ => unreachable!(),
                })
                .next()
                .unwrap()
        })
        .sum::<u32>();

    println!("{sum}");
}
