use std::{str::FromStr, error::Error};

fn main() {
    let commands = parse_file("./02-input.txt").unwrap();

    let mut submarine: SimpleSubmarine = Default::default();
    commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
    println!("Result Simple: {}", submarine.horizontal * submarine.depth);

    let mut submarine: ComplicatedSubmarine = Default::default();
    commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
    println!("Result Complicated: {}", submarine.horizontal * submarine.depth);
}

pub trait Submarine {
    fn make_move(&mut self, command: &Command, amount: u32);
}

struct ComplicatedSubmarine {
    pub horizontal: u32,
    pub aim: u32,
    pub depth: u32
}

impl Submarine for ComplicatedSubmarine {
    fn make_move(&mut self, command: &Command, amount: u32) {
        match command {
            Command::Forward => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            },
            Command::Up => { self.aim -= amount; },
            Command::Down => { self.aim += amount; }
        }
    }
}

pub struct SimpleSubmarine {
    pub horizontal: u32,
    pub depth: u32
}

impl Submarine for SimpleSubmarine {
    fn make_move(&mut self, command: &Command, amount: u32) {
        match command {
            Command::Forward => { self.horizontal += amount; },
            Command::Up => { self.depth -= amount; },
            Command::Down => { self.depth += amount; }
        }
    }
}

pub enum Command {
    Forward,
    Up,
    Down
}

fn parse_file(filename: &str) -> Result<Vec<(Command, u32)>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename).expect("File not found");

    let commands = file.lines().map(|line| {
        let mut words = line.split(" ");
        let command: Command = words.next().expect("No command in line").parse().unwrap();
        let amount: u32 = words.next().expect("No amount in line").parse().expect("Could not parse amount as integer");
        (command, amount)
    }).collect::<Vec<(Command, u32)>>();

    Ok(commands)
}

impl Default for SimpleSubmarine {
    fn default() -> Self {
        Self { horizontal: 0, depth: 0 }
    }
}

impl Default for ComplicatedSubmarine {
    fn default() -> Self {
        Self { horizontal: 0, aim: 0, depth: 0 }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(format!("'{}' is not a valid command", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SimpleSubmarine, parse_file, Submarine, ComplicatedSubmarine};

    #[test]
    fn test_simple_example() {
        let commands = parse_file("./02-example-input.txt").unwrap();

        let mut submarine: SimpleSubmarine = Default::default();
        commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
        assert_eq!(15, submarine.horizontal);
        assert_eq!(10, submarine.depth);
    }

    #[test]
    fn test_simple() {
        let commands = parse_file("./02-input.txt").unwrap();

        let mut submarine: SimpleSubmarine = Default::default();
        commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
        assert_eq!(1451208, submarine.horizontal * submarine.depth);
    }

    #[test]
    fn test_complicated_example() {
        let commands = parse_file("./02-example-input.txt").unwrap();

        let mut submarine: ComplicatedSubmarine = Default::default();
        commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
        assert_eq!(15, submarine.horizontal);
        assert_eq!(60, submarine.depth);
    }

    #[test]
    fn test_complicated() {
        let commands = parse_file("./02-input.txt").unwrap();

        let mut submarine: ComplicatedSubmarine = Default::default();
        commands.iter().for_each(|(command, amount)| { submarine.make_move(&command, *amount)});
        assert_eq!(1620141160, submarine.horizontal * submarine.depth);
    }
}