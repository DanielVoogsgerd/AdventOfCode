use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

const PILE_COUNT: usize = 9;
const INIT_PILE_MAX_HEIGHT: usize = 8;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut segments = value.split_whitespace();

        let amount = segments
            .nth(1)
            .ok_or("Could not find amount")?
            .parse::<usize>()? - 1;
        let from = segments
            .nth(1)
            .ok_or("Could not find from pile")?
            .parse::<usize>()? - 1;
        let to = segments
            .nth(1)
            .ok_or("Could not find to pile")?
            .parse::<usize>()? - 1;

        Ok(Instruction { amount, from, to })
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let inp = read_to_string("./data/05.txt").expect("Could not load data file");
    let mut lines = inp.lines();

    let mut piles: [VecDeque<char>; PILE_COUNT] = Default::default();

    for line in lines.by_ref().take(INIT_PILE_MAX_HEIGHT) {
        for (i, mut segment) in line.chars().chunks(4).into_iter().enumerate() {
            let content = segment.nth(1).unwrap();
            if content == ' ' {
                continue;
            }
            let pile = piles.get_mut(i).unwrap();
            pile.push_front(content);
        }
    }

    let lines = lines.skip_while(|line| !line.is_empty()).skip(1);

    for instruction in lines.filter_map(|line| Instruction::try_from(line).ok()) {
	let index = piles[instruction.from].len() - instruction.amount - 1;
	let grabbed = piles[instruction.from].split_off(index);
	piles[instruction.to].extend(&mut grabbed.into_iter().rev());
    }

    print!("Part one: ");
    for pile in &piles {
	print!("{}", pile.iter().last().unwrap())
    }
    print!("\n");
}

fn part_two() {
    let inp = read_to_string("./data/05.txt").expect("Could not load data file");
    let mut lines = inp.lines();

    let mut piles: [VecDeque<char>; PILE_COUNT] = Default::default();

    for line in lines.by_ref().take(INIT_PILE_MAX_HEIGHT) {
        for (i, mut segment) in line.chars().chunks(4).into_iter().enumerate() {
            let content = segment.nth(1).unwrap();
            if content == ' ' {
                continue;
            }
            let pile = piles.get_mut(i).unwrap();
            pile.push_front(content);
        }
    }

    let lines = lines.skip_while(|line| !line.is_empty()).skip(1);

    for instruction in lines.filter_map(|line| Instruction::try_from(line).ok()) {
	let index = piles[instruction.from].len() - instruction.amount - 1;
	let mut grabbed = piles[instruction.from].split_off(index);
	piles[instruction.to].append(&mut grabbed);
    }

    print!("Part two: ");
    for pile in &piles {
	print!("{}", pile.iter().last().unwrap())
    }
    print!("\n");
}
