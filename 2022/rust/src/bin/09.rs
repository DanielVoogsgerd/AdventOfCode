use std::{
    collections::{BTreeSet, HashSet},
    fs::read_to_string,
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete, sequence::separated_pair, IResult,
};

use nom_supreme::ParserExt;

#[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i64, i64)> for Coord {
    fn from(val: (i64, i64)) -> Self {
        Self { x: val.0, y: val.1 }
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let data = read_to_string("./data/09.txt").expect("Could not read data file");
    let answer = part_one(&data);
    println!("Part one: {answer}");
    let answer = part_two(&data);
    println!("Part two: {answer}");
}

fn part_one(data: &str) -> usize {
    let instructions: Vec<Instruction> = data
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let mut tail_positions = BTreeSet::new();

    let mut head: Coord = (0, 0).into();
    let mut tail: Coord = (0, 0).into();

    tail_positions.insert(tail);

    for instruction in instructions {
        let rel = instruction.to_rel_coord();
        head += rel;

        loop {
            // println!("Head: {head:?}; Tail: {tail:?}");
            let rel = head - tail;
            let rel_x = rel.x.abs();
            let rel_y = rel.y.abs();
            if rel_x <= 1 && rel_y <= 1 {
                break;
            }

            let mut new_tail = tail.clone();

            if rel_x >= 1 {
                new_tail.x += rel.x / rel_x;
            }

            if rel_y >= 1 {
                new_tail.y += rel.y / rel_y;
            }

            tail_positions.insert(new_tail);
            tail = new_tail;
        }
    }

    tail_positions.len()
}

fn part_two(data: &str) -> usize {
    let instructions: Vec<Instruction> = data
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let mut tail_positions = HashSet::new();

    const ROPE_LEN: usize = 10;

    tail_positions.insert(Default::default());
    let mut knots: [Coord; ROPE_LEN] = Default::default();

    for instruction in instructions {
        let rel = instruction.to_rel_coord();

        knots[0] += rel;

        'outer: loop {
	    let mut moved: [bool; ROPE_LEN] = [false; ROPE_LEN];
            for i in 1..ROPE_LEN {
                let lead_knot = knots[i - 1];
                let knot = knots[i];
                let rel = lead_knot - knot;
                let rel_x = rel.x.abs();
                let rel_y = rel.y.abs();

                let mut new_knot = knot.clone();
		if rel_x <= 1 && rel_y <= 1 {
		    continue;
		}

                if rel_x >= 1 {
		    moved[i] = true;
                    new_knot.x += rel.x / rel_x;
                }

                if rel_y >= 1 {
		    moved[i] = true;
                    new_knot.y += rel.y / rel_y;
                }

                if i == ROPE_LEN - 1 {
                    // dbg!(new_knot);
                    tail_positions.insert(new_knot);
                }
                knots[i] = new_knot;
            }

	    if !moved.into_iter().any(|x| x) {
		break;
	    }
        }
    }

    tail_positions.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Instruction {
    fn to_rel_coord(&self) -> Coord {
        match self.direction {
            Direction::Up => (0, self.distance).into(),
            Direction::Right => (self.distance, 0).into(),
            Direction::Left => (-self.distance, 0).into(),
            Direction::Down => (0, -self.distance).into(),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, distance) =
            separated_pair(parse_direction, complete::char(' '), complete::i64)(value)
                .or_else(|_| Err("Could not parse instruction"))?
                .1;

        Ok(Instruction {
            direction,
            distance,
        })
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        tag("L").value(Direction::Left),
        tag("R").value(Direction::Right),
        tag("U").value(Direction::Up),
        tag("D").value(Direction::Down),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::try_from("R 8").unwrap(),
            Instruction {
                direction: Direction::Right,
                distance: 8
            }
        )
    }

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/09-example.txt").expect("Could not read example data file");
        assert_eq!(part_one(&data), 13);
    }
    #[test]
    fn test_part_two() {
        let data =
            read_to_string("./data/09-example.txt").expect("Could not read example data file");
        assert_eq!(part_two(&data), 1);
    }
    #[test]
    fn test_part_two_larger() {
        let data =
            read_to_string("./data/09-example2.txt").expect("Could not read example data file");
        assert_eq!(part_two(&data), 36);
    }
}
