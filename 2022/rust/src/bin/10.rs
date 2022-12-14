// extern crate nom;

use std::fmt::Display;
use std::fs::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::error::ErrorKind;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut cycle = 0;
    let mut x = 1;
    let mut acc = 0;

    let data = read_to_string("./data/10.txt").expect("Could not read datafile");

    let instructions = data
        .lines()
        .filter_map(|line| parse_instruction(line).ok())
        .map(|(_res, instruction)| instruction);

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                let l = (cycle - 20) % 40;
                if l == 0 {
                    acc += cycle * x;
                }
            }
            Instruction::Addx(val) => {
                cycle += 2;
                let l = (cycle - 20) % 40;
                if l == 0 || l == 1 {
                    acc += (cycle - l) * x;
                }
                x += val;
            }
        }
    }

    println!("{acc}");
}

struct CRT {
    data: Vec<bool>,
    height: usize,
    width: usize,
}

impl CRT {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![false; width * height],
            height,
            width,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize) {
        self.data[y * self.width + x] = true;
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get_pixel(x, y) { '#' } else { ' ' })?;
            }
            writeln!(f, "")?;
        }

        write!(f, "")
    }
}

/// A non growable fully filled ringbuf
struct RingBuf<T> {
    data: Vec<T>,
    width: usize,
    start: usize,
}

impl<T> RingBuf<T> {
    fn new(width: usize, filled: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: (0..width).map(|_| filled.clone()).collect(),
            width,
            start: 0,
        }
    }

    fn set(&mut self, index: usize, val: T) {
        self.data[index] = val
    }

    fn get(&self, index: usize) -> &T {
        &self.data[((index as isize - self.start as isize).rem_euclid(self.width as isize)) as usize]
    }

    fn rotate(&mut self, val: i32) {
        self.start = ((self.start as isize + val as isize) % self.width as isize) as usize;
    }
}

fn part_two() {
    let mut cycle = 0;

    let data = read_to_string("./data/10.txt").expect("Could not read datafile");

    let mut sprite = RingBuf::new(40, false);
    sprite.set(0, true);
    sprite.set(1, true);
    sprite.set(2, true);

    let mut crt = CRT::new(40, 6);

    let instructions = data
        .lines()
        .filter_map(|line| parse_instruction(line).ok())
        .map(|(_res, instruction)| instruction);

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                if *sprite.get(cycle) {
                    crt.data[cycle] = true;
                }
                cycle += 1;
            }
            Instruction::Addx(val) => {
                for _i in 0..2 {
                    if *sprite.get(cycle) {
                        crt.data[cycle] = true;
                    }
                    cycle += 1;
                }
                sprite.rotate(val);
            }
        }
    }

    println!("{crt}");
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    Nom(I, ErrorKind),
    ParseIntError(std::num::ParseIntError),
}

impl<I> nom::error::ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<std::num::ParseIntError> for CustomError<&str> {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

fn parse_instruction(line: &str) -> IResult<&str, Instruction> {
    let noop_parser = tag("noop");
    let addx_parser = tag("addx");

    let mut parser = alt((noop_parser, addx_parser));

    let (res, instruction_str) = parser(line)?;
    let (res, instruction) = match instruction_str {
        "noop" => (res, Instruction::Noop),
        "addx" => {
            let (res, number) = parse_number(res.trim())?;
            (res, Instruction::Addx(number))
        }
        _ => panic!("Invalid instruction encountered"),
    };

    Ok((res, instruction))
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let parser = recognize(tuple((opt(char('-')), digit1)));
    let mut parser = map_res(parser, |s: &str| s.parse::<i32>());

    parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_noop() {
        assert_eq!(parse_instruction("noop"), Ok(("", Instruction::Noop)));
    }
    #[test]
    fn test_parse_addx() {
        assert_eq!(
            parse_instruction("addx 12"),
            Ok(("", Instruction::Addx(12)))
        )
    }
}
