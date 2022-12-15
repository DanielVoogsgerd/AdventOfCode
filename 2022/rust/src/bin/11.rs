use std::{collections::VecDeque, fs::read_to_string};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::tuple, IResult,
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("./data/11.txt").expect("Could not read datafile");

    let segments = data.split("\n\n");

    let mut monkeys = segments
        .filter_map(|segment| Monkey::try_from(segment).ok())
        .collect::<Vec<_>>();

    let mut inspection_count = vec![0; monkeys.len()];

    let mut new_items_per_monkey = vec![vec![]; monkeys.len()];
    for _round in 0..20 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey
                .starting_items
                .extend(new_items_per_monkey[i].drain(..));
            while let Some(item) = monkey.starting_items.pop_front() {
                let new_item = (*monkey.operation)(item) / 3;

		inspection_count[i] += 1;
                let new_monkey = if new_item % monkey.divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                new_items_per_monkey[new_monkey].push(new_item);
            }
        }
    }
    inspection_count.sort();
    let shenanigans = inspection_count.iter().rev().take(2).product::<usize>();
    println!("{shenanigans}")
}

fn part_two() {
    let data = read_to_string("./data/11.txt").expect("Could not read datafile");

    let segments = data.split("\n\n");

    let mut monkeys = segments
        .filter_map(|segment| Monkey::try_from(segment).ok())
        .collect::<Vec<_>>();

    let product = monkeys.iter().map(|monkey| monkey.divisible_by).product::<usize>();

    let mut inspection_count = vec![0; monkeys.len()];

    let mut new_items_per_monkey = vec![vec![]; monkeys.len()];
    for _round in 0..10_000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey
                .starting_items
                .extend(new_items_per_monkey[i].drain(..));
            while let Some(item) = monkey.starting_items.pop_front() {
                let new_item = (*monkey.operation)(item) % product;

		inspection_count[i] += 1;
                let new_monkey = if new_item % monkey.divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                new_items_per_monkey[new_monkey].push(new_item);
            }
        }
    }
    inspection_count.sort();
    let shenanigans = inspection_count.iter().rev().take(2).product::<usize>();
    println!("{shenanigans}")
}

struct Monkey<'a> {
    starting_items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize + 'a>,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

fn parse_expression<'a>(input: &'a str) -> IResult<&'a str, Box<dyn Fn(usize) -> usize + 'a>> {
    let mut parser = map(
        tuple((
            tag("old"),
            alt((tag(" + "), tag(" - "), tag(" * "))),
            alt((tag("old"), digit1)),
        )),
        |(_lhs, op, rhs)| -> Box<dyn Fn(usize) -> usize> {
            match op {
                " + " => Box::new(move |old| {
                    let rhs = match rhs {
                        "old" => old,
                        num => num.parse::<usize>().unwrap(),
                    };
                    old + rhs
                }),
                " - " => Box::new(move |old| {
                    let rhs = match rhs {
                        "old" => old,
                        num => num.parse::<usize>().unwrap(),
                    };
                    old - rhs
                }),
                " * " => Box::new(move |old| {
                    let rhs = match rhs {
                        "old" => old,
                        num => num.parse::<usize>().unwrap(),
                    };
                    old * rhs
                }),
                _ => unreachable!(),
            }
        },
    );
    let (remaining, matched) = parser(input)?;
    Ok((remaining, matched))
}

impl<'a> TryFrom<&'a str> for Monkey<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut iter = value.lines().skip(1);
        let (_, starting_items_str) = iter
            .next()
            .ok_or("No starting items found")?
            .split_once(": ")
            .ok_or("No values found for starting items")?;


        let starting_items = starting_items_str
            .split(", ")
            .filter_map(|number| number.parse::<usize>().ok())
            .collect();

        let (_, operation_str) = iter
            .next()
            .ok_or("No operation found")?
            .split_once(": ")
            .ok_or("No value found for operation")?;

        let (_lhs, rhs) = operation_str
            .split_once(" = ")
            .ok_or("No operation found")?;

        let (_, operation) =
            parse_expression(rhs).or_else(|_| Err("Could not parse expression"))?;

        let divisible_by = iter
            .next()
            .ok_or("No divisible by found")?
            .split_whitespace()
            .last()
            .ok_or("No values found for divisible by")?
            .parse::<usize>()?;

        let if_true = iter
            .next()
            .ok_or("No if true found")?
            .split_whitespace()
            .last()
            .ok_or("No values found for if true")?
            .parse::<usize>()?;

        let if_false = iter
            .next()
            .ok_or("No if false found")?
            .split_whitespace()
            .last()
            .ok_or("No values found for if false")?
            .parse::<usize>()?;

        Ok(Monkey {
            starting_items,
            operation,
            divisible_by,
            if_true,
            if_false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let expression = parse_expression("old + 3").unwrap().1;
        assert_eq!(expression(1), 4)
    }
}
