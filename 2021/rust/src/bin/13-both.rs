use std::{collections::BTreeSet, error::Error};

const FILLED_CHAR: char = '█';
const EMPTY_CHAR: char  = ' ';

fn main() {
    let (mut board, instructions) = parse_file("./13-input.txt").unwrap();

    // First star
    board.perform_instruction(&instructions[0]);
    println!("Result part one: {:?}", board.data.len());

    // Second star
    for instruction in &instructions {
        board.perform_instruction(instruction);
    }

    println!("Result part 2:\n{}", board.print_board());
}

fn parse_file(filename: &str) -> Result<(Board, Vec<(String, usize)>), Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    let mut lines = file.lines();

    let coords: BTreeSet<(usize, usize)>= lines.by_ref().take_while(|&x| x != "").map(|line| {
        let parts: Vec<usize> = line.split(",").map(|x| {x.parse::<usize>().expect("Could not parse as int")}).collect();
        return (parts[0], parts[1])
    }).collect();

    let instructions: Vec<(String, usize)> = lines.by_ref().map(|line| {
        let mut parts = line.split(" ").nth(2).expect("Instruction too short").split("=");
        return (String::from(parts.next().unwrap()), parts.next().unwrap().parse::<usize>().expect("Could not parse fold location to int"))
    }).collect();

    let board = Board { data: coords };

    Ok((board, instructions))
}

struct Board {
    data: BTreeSet<(usize, usize)>
}

impl Board {
    fn perform_instruction(&mut self, instruction: &(String, usize)) {
        if instruction.0 == "x" {
            let (mut keep, fold): (BTreeSet<(usize, usize)>, BTreeSet<(usize, usize)>) = self.data.iter().partition(|(x, _y)| {
                x < &instruction.1
            });
            fold.iter().map(|(x, y)| {
                (2 * instruction.1 - x, *y)
            }).for_each(|(x, y)| {
                keep.insert((x, y));
            });
            self.data = keep;
        } else {
            let (mut keep, fold): (BTreeSet<(usize, usize)>, BTreeSet<(usize, usize)>) = self.data.iter().partition(|(_x, y)| {
                y < &instruction.1
            });
            fold.iter().map(|(x, y)| {
                (*x, 2 * instruction.1 - y)
            }).for_each(|(x, y)| {
                keep.insert((x, y));
            });
            self.data = keep;
        }
    }

    fn print_board(&self) -> String {
        let mut output = String::new();
        let width = self.data.iter()
            .map(|(x, _y)| { x })
            .max()
            .expect("No maximum x found") + 1;

        let height = self.data.iter()
            .map(|(_x, y)| { y })
            .max()
            .expect("No maximum y found") + 1;

        for y in 0..height {
            for x in 0..width {
                let character = if self.data.contains(&(x, y)) { FILLED_CHAR } else { EMPTY_CHAR };
                output.push(character);
            }
            output.push('\n');
        }
        output
    }
}