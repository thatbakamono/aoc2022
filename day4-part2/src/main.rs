use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let count = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();

            let (first_begin, first_end) = first.split_once('-').unwrap();
            let (second_begin, second_end) = second.split_once('-').unwrap();

            let first_begin = first_begin.parse::<usize>().unwrap();
            let first_end = first_end.parse::<usize>().unwrap();
            let second_begin = second_begin.parse::<usize>().unwrap();
            let second_end = second_end.parse::<usize>().unwrap();

            let first = first_begin..=first_end;
            let second = second_begin..=second_end;

            (first, second)
        })
        .filter(|(first, second)| {
            // One of the ranges is contained within the second one
            if (first.start() <= second.start() && first.end() >= second.end())
                || (second.start() <= first.start() && second.end() >= first.end())
            {
                return true;
            }

            // Ranges have some intersection
            if (first.start() < second.start()
                && first.end() >= second.start()
                && first.end() < second.end())
                || (second.start() < first.start()
                    && second.end() >= first.start()
                    && second.end() < first.end())
            {
                return true;
            }

            false
        })
        .count();

    println!("{count}");
}
