// struct Rock<const N: usize> {
//     filled: Vec<usize>
// }

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Invalid direction character".into()),
        }
    }
}

impl Direction {
    fn to_stride(&self, width: isize) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Down => -width,
        }
    }
}

use std::{collections::{BTreeSet, HashMap}, fmt::Display, fs::read_to_string};

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    jet: usize,
    rock: usize,
    floor: Vec<isize>,
}

struct Chamber {
    active: Vec<isize>,
    filled: BTreeSet<isize>,
    top_row: isize,
    width: isize,
    bottom: Vec<isize>,
}

impl Chamber {
    fn new(width: isize) -> Self {
        Self {
            active: vec![],
            filled: (0..width).collect(),
            top_row: 0,
            width,
            bottom: (0..width).map(|_| 0).collect(),
        }
    }

    fn get_rocks() -> [Vec<isize>; 5] {
        [
            vec![0, 1, 2, 3],
            vec![1, 7, 8, 9, 15],
            vec![0, 1, 2, 9, 16],
            vec![0, 7, 14, 21],
            vec![0, 1, 7, 8],
        ]
    }

    fn add_rock(&mut self, shape: impl IntoIterator<Item = isize>) {
        self.active = shape
            .into_iter()
            .map(|x| x + 2 + (self.top_row + 4) * self.width)
            .collect();
    }

    fn valid_move(&self, direction: Direction) -> bool {
        if match direction {
            Direction::Left => self.active.iter().any(|index| index % self.width == 0),
            Direction::Right => self
                .active
                .iter()
                .any(|index| index % self.width == self.width - 1),
            _ => false,
        } {
            return false;
        }

        let stride = direction.to_stride(self.width);

        self.active
            .iter()
            .all(|&index| !self.filled.contains(&(index + stride)))
    }

    fn r#move(&mut self, direction: Direction) {
        for index in self.active.iter_mut() {
            *index += direction.to_stride(self.width)
        }
    }

    fn solidify(&mut self) {
        for index in self.active.drain(..) {
            let row = index / self.width;
            self.top_row = isize::max(row, self.top_row);
            let column = (index % self.width) as usize;
            self.bottom[column] = isize::max(self.bottom[column], row);
            self.filled.insert(index);
        }
        self.filled = self
            .filled
            .split_off(&(*self.bottom.iter().min().unwrap() * self.width));
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (self.bottom.iter().cloned().min().unwrap() - 3..self.top_row + 10).rev() {
            let data = (0..self.width)
                .map(|column| {
                    let index = self.width * row + column;
                    if self.filled.contains(&index) {
                        '#'
                    } else if self.active.contains(&index) {
                        '@'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            writeln!(f, "{data}")?;
        }

        write!(f, "")
    }
}

fn main() {
    let data = read_to_string("./data/17.txt").expect("Could not read data file");
    let answer = part_one(&data);
    println!("{answer}");
    let answer = part_two(&data);
    println!("{answer}");
}

fn part_one(data: &str) -> usize {
    let wind_dirs: Vec<Direction> = data.chars().filter_map(|chr| chr.try_into().ok()).collect();

    let rocks = Chamber::get_rocks();
    let mut chamber = Chamber::new(7);

    let mut i = 0;

    for rock_index in 0..2022 {
        chamber.add_rock(rocks[rock_index % rocks.len()].clone());

        loop {
            let current_move = wind_dirs[i % wind_dirs.len()];
            if chamber.valid_move(current_move) {
                chamber.r#move(current_move);
            }

            i += 1;

            if chamber.valid_move(Direction::Down) {
                chamber.r#move(Direction::Down);
            } else {
                chamber.solidify();
                break;
            }
        }
    }

    println!("{:?}", chamber.bottom);
    println!("{chamber}");

    chamber.top_row as usize
}

fn part_two(data: &str) -> usize {
    let wind_dirs: Vec<Direction> = data.chars().filter_map(|chr| chr.try_into().ok()).collect();

    let rocks = Chamber::get_rocks();
    let mut chamber = Chamber::new(7);

    let mut i = 0;
    let big_boi = 1_000_000_000_000;

    let mut dict: HashMap<Key, (usize, isize)> = Default::default();

    let mut rock_index = 0;

    let mut skipped_height = 0;

    while rock_index < big_boi {
        chamber.add_rock(rocks[rock_index % rocks.len()].clone());

        loop {
            let current_move = wind_dirs[i % wind_dirs.len()];
            if chamber.valid_move(current_move) {
                chamber.r#move(current_move);
            }

            i += 1;

            if chamber.valid_move(Direction::Down) {
                chamber.r#move(Direction::Down);
            } else {
                chamber.solidify();
                break;
            }
        }

	let key = Key {
	    jet: i % wind_dirs.len(),
	    rock: rock_index % rocks.len(),
	    floor: chamber.bottom.iter().cloned().map(|x| x - chamber.bottom.iter().min().unwrap()).collect(),
	};

	if skipped_height == 0 {
	    if let Some((old_rock_index, old_top_row)) = dict.get(&key) {
		let cycle_length = rock_index - old_rock_index;
		let cycle_height = chamber.top_row - old_top_row;
		let remaining_cycles = (big_boi - rock_index) / cycle_length;

		skipped_height = remaining_cycles * cycle_height as usize;
		rock_index += cycle_length * remaining_cycles;

	    } else {
		dict.insert(key, (rock_index, chamber.top_row));
	    }
	}

	rock_index += 1;
    }

    println!("{:?}", chamber.bottom);
    println!("{chamber}");

    chamber.top_row as usize + skipped_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part_one(&data), 3068);
    }

    #[test]
    fn test_part_two() {
        let data = String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part_two(&data), 1514285714288);
    }

    #[test]
    fn test_valid_move_left() {
        let rocks = Chamber::get_rocks();
        let mut chamber = Chamber::new(7);
        chamber.add_rock(rocks[0].clone());
        chamber.r#move(Direction::Down);
        chamber.r#move(Direction::Left);
        assert!(chamber.valid_move(Direction::Left));
        chamber.r#move(Direction::Left);
        println!("{chamber}");
        assert!(!chamber.valid_move(Direction::Left));
    }
}
