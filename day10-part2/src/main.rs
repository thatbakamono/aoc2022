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

    let mut x_register = 1;
    let mut x_pending_value = 0;
    let mut active_instruction = None;

    let mut display = [['.'; 40]; 6];

    let mut x = 0usize;
    let mut y = 0usize;

    while !instructions.is_empty() || active_instruction.is_some() {
        x_register += x_pending_value;
        x_pending_value = 0isize;

        if x == 40 {
            x = 0;
            y += 1;

            if y == 6 {
                y = 0;
            }
        }

        if (x_register - 1) as usize == x
            || x_register as usize == x
            || (x_register + 1) as usize == x
        {
            display[y][x] = '#';
        }

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

        x += 1;
    }

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", display[y][x]);
        }

        println!();
    }
}

enum Instruction {
    AddX { value: isize },
    Noop,
}
