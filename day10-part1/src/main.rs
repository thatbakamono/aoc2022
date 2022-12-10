use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut instructions = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();

            match parts[0] {
                "addx" => {
                    let value = parts[1].parse::<isize>().unwrap();

                    Instruction::AddX { value }
                }
                "noop" => Instruction::Noop,
                _ => panic!("Invalid instruction"),
            }
        })
        .collect::<VecDeque<_>>();

    let mut cycle = 0;
    let mut x_register = 1;
    let mut x_pending_value = 0;
    let mut active_instruction = None;
    let mut total_signal_strength = 0;

    while !instructions.is_empty() || active_instruction.is_some() {
        cycle += 1;

        x_register += x_pending_value;
        x_pending_value = 0;

        if let Some(instruction) = active_instruction {
            match instruction {
                Instruction::Noop => (),
                Instruction::AddX { value } => x_pending_value = value,
            }

            active_instruction = None;
        } else {
            let instruction = instructions.pop_front().unwrap();

            match instruction {
                Instruction::Noop => (),
                Instruction::AddX { .. } => active_instruction = Some(instruction),
            }
        }

        if cycle == 20 || (cycle - 20) % 40 == 0 {
            total_signal_strength += cycle * x_register;
        }
    }

    println!("{total_signal_strength}");
}

enum Instruction {
    AddX { value: isize },
    Noop,
}
