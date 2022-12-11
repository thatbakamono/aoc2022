use chumsky::prelude::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut monkeys = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .chunks(6)
        .into_iter()
        .map(|mut chunks| {
            chunks.next().unwrap();

            let starting_items = chunks.next().unwrap();
            let operation = chunks.next().unwrap();
            let test = chunks.next().unwrap();
            let if_true = chunks.next().unwrap();
            let if_false = chunks.next().unwrap();

            let items = starting_items.trim()[16..]
                .split(", ")
                .map(|number| {
                    let worry_level = number.parse::<usize>().unwrap();

                    Item { worry_level }
                })
                .collect::<VecDeque<_>>();

            let operation = Expression::parse(&operation.trim()[11..]);
            let divisible_by_test = test.trim()[19..].parse::<usize>().unwrap();
            let monkey_to_throw_to_if_true = if_true.trim()[25..].parse::<usize>().unwrap();
            let monkey_to_throw_to_if_false = if_false.trim()[26..].parse::<usize>().unwrap();

            Monkey {
                items,
                operation,
                divisible_by_test,
                monkey_to_throw_to_if_true,
                monkey_to_throw_to_if_false,
                inspections: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut context = HashMap::new();

    for _round in 0..20 {
        for index in 0..monkeys.len() {
            while !monkeys[index].items.is_empty() {
                let mut item = monkeys[index].items.pop_front().unwrap();

                context.insert(String::from("old"), item.worry_level);
                context.insert(String::from("new"), 0);

                monkeys[index].operation.evaluate(&mut context);

                item.worry_level = *context.get("new").unwrap() / 3;

                let monkey_to_throw_to_if_true = monkeys[index].monkey_to_throw_to_if_true;
                let monkey_to_throw_to_if_false = monkeys[index].monkey_to_throw_to_if_false;

                if item.worry_level % monkeys[index].divisible_by_test == 0 {
                    monkeys[monkey_to_throw_to_if_true].items.push_back(item);
                } else {
                    monkeys[monkey_to_throw_to_if_false].items.push_back(item);
                }

                monkeys[index].inspections += 1;
            }
        }
    }

    let monkey_business_level = monkeys
        .into_iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>();

    println!("{monkey_business_level}");
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Expression,
    divisible_by_test: usize,
    monkey_to_throw_to_if_true: usize,
    monkey_to_throw_to_if_false: usize,
    inspections: usize,
}

struct Item {
    worry_level: usize,
}

enum Expression {
    Value(usize),
    Variable(String),
    Set {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Multiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

impl Expression {
    fn parse(input: &str) -> Expression {
        recursive(
            |expression: Recursive<'_, char, Expression, Simple<char>>| {
                let value = filter(|character: &char| character.is_ascii_digit())
                    .repeated()
                    .at_least(1)
                    .map(|characters| {
                        characters
                            .into_iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap_or(0)
                    })
                    .map(Expression::Value);

                let variable = filter(|character: &char| character.is_ascii_alphabetic())
                    .repeated()
                    .at_least(1)
                    .collect()
                    .map(Expression::Variable);

                variable
                    .then_ignore(just('=').padded())
                    .then(expression.clone())
                    .map(|(lhs, rhs)| Expression::Set {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    })
                    .or(variable
                        .then_ignore(just('+').padded())
                        .then(expression.clone())
                        .map(|(lhs, rhs)| Expression::Add {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }))
                    .or(variable
                        .then_ignore(just('*').padded())
                        .then(expression.clone())
                        .map(|(lhs, rhs)| Expression::Multiply {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }))
                    .or(variable)
                    .or(value)
            },
        )
        .parse(input)
        .unwrap()
    }

    fn evaluate(&self, context: &mut HashMap<String, usize>) -> usize {
        match self {
            Expression::Value(value) => *value,
            Expression::Variable(variable) => *context.get(variable).unwrap(),
            Expression::Set { lhs, rhs } => match &**lhs {
                Expression::Variable(variable) => {
                    let value = rhs.evaluate(context);

                    context.insert(variable.clone(), value);

                    0
                }
                _ => panic!(),
            },
            Expression::Add { lhs, rhs } => lhs.evaluate(context) + rhs.evaluate(context),
            Expression::Multiply { lhs, rhs } => lhs.evaluate(context) * rhs.evaluate(context),
        }
    }
}
