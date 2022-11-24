use std::{error::Error, convert::TryInto, collections::BTreeSet};

// const OFFSET: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
// const SURROUNINGS: [(isize, isize); 9] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];
const SURROUNINGS: [(isize, isize); 9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

const FILLED_CHAR: char = '█';
const EMPTY_CHAR: char  = ' ';

fn main() {
    let mut board = parse_file("./20-example-input.txt").unwrap();
    for _i in 0..2 {
        board.enhance();
    }
    println!("{}", board.print_board().unwrap());
    println!("Lit pixels: {}", board.data.len());
}

fn parse_file(filename: &str) -> Result<Board, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    let mut lines = file.lines();

    let alg_str = lines.next().ok_or("No algorithm found")?;
    let alg = alg_str.chars().map(|character|character == '#').collect::<Vec<bool>>();
    let alg = alg.try_into().or_else(|_|Err("Could not convert algorithm into an array of size 512"))?;

    lines.next().ok_or("No newline between algorithm and image")?;

    let data = lines.enumerate().map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, character)|{
            if character == '#' { Some((x as isize, y as isize)) } else { None }
        })
    }).flatten().collect();

    let board = Board {
        data: data,
        algorithm: alg,
        even_generation: true
    };

    Ok(board)
}

pub struct Board {
    pub algorithm: [bool; 512],
    pub data: BTreeSet<(isize, isize)>,
    even_generation: bool
}

impl Board {
    pub fn enhance(&mut self) {
        let currently_enabled = self.data.iter().map(|(x, y)| {
            self.surroundings(*x, *y).into_iter()
        }).flatten().collect::<BTreeSet<(isize, isize)>>();

        let new = currently_enabled.into_iter().filter(|coord| {
            let index = self.get_algorithm_index(coord.0, coord.1);
            self.even_generation ^ self.algorithm[index]
        }).collect::<BTreeSet<(isize, isize)>>();

        self.data = new;
        self.even_generation = !self.even_generation;
    }

    fn get_algorithm_index(&self, x: isize, y: isize) -> usize {
        self.surroundings(x, y).into_iter()
            .map(|field| { self.data.contains(&field) })
            .fold(0, |acc, cur| { 2*acc + if !self.even_generation ^ cur {1} else {0} })
    }

    pub fn surroundings(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        SURROUNINGS.iter().map(|(dx, dy)| {
            (x + *dx, y + *dy)
        }).collect()
    }

    fn print_board(&self) -> Result<String, &str> {
        let mut output = String::new();
        let min_x = self.data.iter()
            .map(|(x, _y)| { x })
            .min()
            .ok_or("No minimum x found")? - 1;

        let max_x = self.data.iter()
            .map(|(x, _y)| { x })
            .max()
            .ok_or("No maximum x found")? + 1;

        let min_y = self.data.iter()
            .map(|(_x, y)| { y })
            .min()
            .ok_or("No minimum y found")? - 1;

        let max_y = self.data.iter()
            .map(|(_x, y)| { y })
            .max()
            .ok_or("No maximum y found")? + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let character = if self.data.contains(&(x, y)) { FILLED_CHAR } else { EMPTY_CHAR };
                output.push(character);
            }
            output.push('\n');
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    #[test]
    fn test_example_scan() {
        let board = parse_file("./20-example-input.txt").unwrap();
        assert_eq!(board.get_algorithm_index(2, 2), 34);
    }

    #[test]
    fn test_part_1() {
        let mut board = parse_file("./20-input.txt").unwrap();
        for _i in 0..2 {
            board.enhance();
        }
        
        assert_eq!(5291, board.data.len());
    }

    #[test]
    fn test_part_2() {
        let mut board = parse_file("./20-input.txt").unwrap();
        for _i in 0..50 {
            board.enhance();
        }
        
        assert_eq!(16665, board.data.len());
    }
}