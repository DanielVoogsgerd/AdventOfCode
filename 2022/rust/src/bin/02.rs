use std::{error::Error, fs::read_to_string};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("./data/02.txt").expect("Could not load datafile");

    let score = data
        .lines()
        .map(|line| {
            let (opponent, me) = line.split_once(' ').unwrap();
            let opponent: Shape = opponent.try_into().unwrap();
            let me = part_one_map(me.chars().next().unwrap()).unwrap();
            let brave_score = me.to_num() + 1;
            let win_score = match me.cmp(&opponent) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 3,
                std::cmp::Ordering::Greater => 6,
            };

            brave_score as usize + win_score
        })
        .sum::<usize>();

    println!("{score}")
}

fn part_two() {
    let data = read_to_string("./data/02.txt").expect("Could not load datafile");

    let shapes = data
        .lines()
        .map(|line| {
	    let (opponent, me) = line.split_once(' ').unwrap();
            let opponent: Shape = opponent.try_into().unwrap();
	    let strat = me.chars().next().unwrap();
            let me = part_two_map(strat, &opponent).unwrap();

            let brave_score = me.to_num() + 1;
            let win_score = match me.cmp(&opponent) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 3,
                std::cmp::Ordering::Greater => 6,
            };
            brave_score as usize + win_score
	})
        .sum::<usize>();

    println!("{shapes}")
}

fn part_one_map(chr: char) -> Result<Shape, Box<dyn Error>> {
    Ok(match chr {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => return Err("Incorrect input char".into()),
    }
    .try_into()?)
}

fn part_two_map(strat: char, opponent: &Shape) -> Result<Shape, Box<dyn Error>> {
    let diff = match strat {
        'X' => 2,
        'Y' => 0,
        'Z' => 1,
        _ => return Err("Incorrect input char".into()),
    };

    Ok(Shape::from_num((opponent.to_num() + diff).rem_euclid(3)))
}

#[derive(PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn to_num(&self) -> isize {
        match self {
            Shape::Rock => 0,
            Shape::Paper => 1,
            Shape::Scissors => 2,
        }
    }

    fn from_num(num: isize) -> Self {
        match num {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Incorrect number for shape"),
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err("Invalid shape"),
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .ok_or("Could not get first char from &str")?
            .try_into()
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.to_num() - other.to_num()).rem_euclid(3) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            2 => std::cmp::Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_test_lose() {
        assert_eq!(part_two_map('X', &Shape::Rock).unwrap(), Shape::Scissors);
        assert_eq!(part_two_map('X', &Shape::Paper).unwrap(), Shape::Rock);
        assert_eq!(part_two_map('X', &Shape::Scissors).unwrap(), Shape::Paper);
    }
    #[test]
    fn test_part_two_test_draw() {
        assert_eq!(part_two_map('Y', &Shape::Rock).unwrap(), Shape::Rock);
        assert_eq!(part_two_map('Y', &Shape::Paper).unwrap(), Shape::Paper);
        assert_eq!(
            part_two_map('Y', &Shape::Scissors).unwrap(),
            Shape::Scissors
        );
    }
    #[test]
    fn test_part_two_test_win() {
        assert_eq!(part_two_map('Z', &Shape::Rock).unwrap(), Shape::Paper);
        assert_eq!(part_two_map('Z', &Shape::Paper).unwrap(), Shape::Scissors);
        assert_eq!(part_two_map('Z', &Shape::Scissors).unwrap(), Shape::Rock);
    }
}
