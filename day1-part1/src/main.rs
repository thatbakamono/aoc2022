use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<usize>().ok());

    let mut biggest_inventory = 0usize;
    let mut current_inventory = 0usize;

    for line in lines {
        if let Some(line) = line {
            current_inventory += line;
        } else {
            if current_inventory > biggest_inventory {
                biggest_inventory = current_inventory;
            }

            current_inventory = 0usize;
        }
    }

    println!("{biggest_inventory}");
}
