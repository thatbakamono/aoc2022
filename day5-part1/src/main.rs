use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut state = State::ParsingState;

    let mut stacks = vec![];
    let mut instructions = vec![];

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            if line.len() > 0 {
                match state {
                    State::ParsingState => {
                        if line.chars().nth(1).unwrap().is_ascii_digit() {
                            state = State::ParsingInstructions;
                        } else {
                            let mut iterator =
                                line.chars().collect::<Vec<_>>().into_iter().peekable();

                            let mut n = 0;

                            while let Some(character) = iterator.next() {
                                if character == ' ' {
                                    iterator.next().expect("Expected ' '");
                                    iterator.next().expect("Expected ' '");

                                    if stacks.len() <= n {
                                        stacks.push(VecDeque::new());
                                    }
                                } else if character == '[' {
                                    let letter = iterator.next().expect("Expected letter");

                                    iterator.next().expect("Expected ']'");

                                    if stacks.len() <= n {
                                        stacks.push(VecDeque::new());
                                    }

                                    stacks[n].push_back(letter);
                                } else {
                                    panic!();
                                }

                                if let Some(character) = iterator.next() {
                                    if character != ' ' {
                                        panic!("Expected ' ', found '{character}'");
                                    }
                                }

                                n += 1;
                            }
                        }
                    }
                    State::ParsingInstructions => {
                        let parts = line.split_terminator(" ").collect::<Vec<_>>();

                        assert_eq!(parts.len(), 6);
                        assert_eq!(parts[0], "move");
                        assert_eq!(parts[2], "from");
                        assert_eq!(parts[4], "to");

                        let count = parts[1].parse::<usize>().expect("Expected number");
                        let from = parts[3].parse::<usize>().expect("Expected number");
                        let to = parts[5].parse::<usize>().expect("Expected number");

                        instructions.push(Instruction { from, to, count });
                    }
                }
            }
        });

    for instruction in instructions {
        for _ in 0..instruction.count {
            let element = stacks[instruction.from - 1]
                .pop_front()
                .expect("Expected element on stack");

            stacks[instruction.to - 1].push_front(element);
        }
    }

    let result = stacks.into_iter().map(|stack| stack[0]).collect::<String>();

    println!("{result}");
}

enum State {
    ParsingState,
    ParsingInstructions,
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}
