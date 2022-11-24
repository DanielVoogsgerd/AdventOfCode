use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
}

#[derive(Debug)]
enum Instruction {
    On,
    Off,
    Toggle,
}

type Coord = (usize, usize);

const STRIDE: usize = 1000;

fn part_one() {
    let data = read_to_string("./data/06.txt").expect("Could not read datafile");

    let instructions = data.lines().map(|line| {
        let mut parts = line.split_whitespace();

        let instruction = match parts.next().expect("invalid format") {
            "turn" => match parts.next().expect("invalid format") {
                "on" => Instruction::On,
                "off" => Instruction::Off,
                _ => panic!("Invalid turn instruction"),
            },
            "toggle" => Instruction::Toggle,
            _ => panic!("Invalid instructions"),
        };
        let loc1 = parts.next().expect("Missing first turn location");
        let loc2 = parts.nth(1).expect("Missing second turn location");

        (
            instruction,
            parse_coord(loc1).expect("Invalid coord 1"),
            parse_coord(loc2).expect("Invalid coord 2"),
        )
    });

    let mut board = [false; 1_000_000];

    for (instruction, loc1, loc2) in instructions {
	for x in loc1.0..=loc2.0 {
	    for y in loc1.1..=loc2.1 {
		let index = get_index(x, y);
		match instruction {
		    Instruction::On => {
			board[index] = true
		    },
		    Instruction::Off => {
			board[index] = false
		    },
		    Instruction::Toggle => {
			board[index] = !board[index]
		    },
		}
	    }
	}
    }

    println!("{}", board.iter().filter(|&&x| x).count());
}

fn get_index(x: usize, y: usize) -> usize {
    y * STRIDE + x
}

fn parse_coord(inp: &str) -> Option<Coord> {
    let mut parts = inp.split(',');
    Some((
        parts.next()?.parse::<usize>().ok()?,
        parts.next()?.parse::<usize>().ok()?,
    ))
}

fn part_two() {
    let data = read_to_string("./data/06.txt").expect("Could not read datafile");

    let instructions = data.lines().map(|line| {
        let mut parts = line.split_whitespace();

        let instruction = match parts.next().expect("invalid format") {
            "turn" => match parts.next().expect("invalid format") {
                "on" => Instruction::On,
                "off" => Instruction::Off,
                _ => panic!("Invalid turn instruction"),
            },
            "toggle" => Instruction::Toggle,
            _ => panic!("Invalid instructions"),
        };
        let loc1 = parts.next().expect("Missing first turn location");
        let loc2 = parts.nth(1).expect("Missing second turn location");

        (
            instruction,
            parse_coord(loc1).expect("Invalid coord 1"),
            parse_coord(loc2).expect("Invalid coord 2"),
        )
    });

    let mut board = [0usize; 1_000_000];

    for (instruction, loc1, loc2) in instructions {
	for x in loc1.0..=loc2.0 {
	    for y in loc1.1..=loc2.1 {
		let index = get_index(x, y);
		match instruction {
		    Instruction::On => {
			board[index] += 1
		    },
		    Instruction::Off => {
			board[index] = usize::max(1, board[index]) - 1
		    },
		    Instruction::Toggle => {
			board[index] += 2
		    },
		}
	    }
	}
    }

    println!("{}", board.iter().sum::<usize>());
}
