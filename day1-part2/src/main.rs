use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<usize>().ok());

    let mut inventories = vec![];
    let mut current_inventory = 0usize;

    for line in lines {
        if let Some(line) = line {
            current_inventory += line;
        } else {
            inventories.push(current_inventory);
            current_inventory = 0usize;
        }
    }

    inventories.sort();

    println!("{}", inventories.into_iter().rev().take(3).sum::<usize>());
}
