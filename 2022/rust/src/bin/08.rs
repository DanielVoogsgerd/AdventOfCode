use std::{collections::HashSet, fs::read_to_string};

fn main() {
    part_one();
    part_two();
}

struct Forest {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
    Left,
    Up,
    Right,
}

impl Direction {
    fn to_stride(&self, width: usize) -> isize {
        match self {
            Direction::Down => width as isize,
            Direction::Left => -1,
            Direction::Up => -1 * (width as isize),
            Direction::Right => 1,
        }
    }

    fn first_row(&self, width: usize, height: usize) -> Vec<usize> {
        match self {
            Direction::Down => (0..width).collect(),
            Direction::Left => (0..height).map(|x| x * width + width - 1).collect(),
            Direction::Up => (0..width).map(|x| x + height * (width - 1)).collect(),
            Direction::Right => (0..height).map(|x| x * width).collect(),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Down,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ]
        .iter()
        .copied()
    }

    fn has_wrapped(&self, index: usize, width: usize, height: usize) -> bool {
        match self {
            Direction::Down => index < width,
            Direction::Left => index % width == width - 1,
            Direction::Up => index >= (height - 1) * width,
            Direction::Right => index % width == 0,
        }
    }
}

impl Forest {
    fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = read_to_string(file)?;
        let width = data.lines().next().unwrap().chars().count();
        let forest = data
            .lines()
            .flat_map(|line| line.chars().map(|chr| chr.to_digit(10).unwrap() as u8))
            .collect::<Vec<_>>();

        Ok(Self {
            height: forest.len() / width,
            width,
            data: forest,
        })
    }

    fn highest_trees_from_direction(&self, direction: Direction) -> HashSet<usize> {
        let stride = direction.to_stride(self.width);
        let first_row = direction.first_row(self.width, self.height);

        let mut highest_trees = HashSet::new();

        // println!("Evaluating direction: {direction:?} (found stride {stride})");
        for start_index in first_row {
            let mut current_index: isize = start_index as isize;
            let mut highest: isize = self.data[current_index as usize] as isize;
            highest_trees.insert(current_index as usize);
            current_index += stride;
            while current_index >= 0
                && current_index < self.data.len() as isize
                && !direction.has_wrapped(current_index as usize, self.width, self.height)
            {
                // println!("Evaluating index: {current_index}");
                if self.data[current_index as usize] as isize > highest {
                    // println!("Highest!");
                    highest_trees.insert(current_index as usize);
                    highest = self.data[current_index as usize] as isize;
                }

                current_index += stride;
            }
        }

        highest_trees
    }
}

fn part_one() {
    let forest = Forest::from_file("./data/08.txt").expect("Could not load forest");
    let mut total = HashSet::new();
    for direction in Direction::iter() {
        total.extend(forest.highest_trees_from_direction(direction).drain());
    }

    println!("{:}", total.len());
}

fn part_two() {
    let forest = Forest::from_file("./data/08.txt").expect("Could not load forest");
    let answer = forest
        .data
        .iter()
        .enumerate()
        .map(|(i, height)| {
            let mut scenic_score = 1;

            for direction in Direction::iter() {
                // println!("\n{direction:?} {i}");
                let mut current_index = i as isize;
                let mut visible_trees = 0;
                let stride = direction.to_stride(forest.width);
                current_index += stride;
                while current_index >= 0
                    && current_index < forest.data.len() as isize
                    && !direction.has_wrapped(current_index as usize, forest.width, forest.height)
                {
                    // println!("{current_index}");
                    visible_trees += 1;
                    if forest.data[current_index as usize] >= *height {
                        break;
                    }

                    current_index += stride;
                }
                scenic_score *= visible_trees;
            }

            // println!("Scenic score: {scenic_score}");
            scenic_score
        })
        .max()
        .expect("Could not find best scenic score");

    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_row_up() {
        let first_row = Direction::Up.first_row(10, 10);
        assert_eq!(first_row, [90, 91, 92, 93, 94, 95, 96, 97, 98, 99])
    }
    #[test]
    fn test_first_row_down() {
        let first_row = Direction::Down.first_row(10, 10);
        assert_eq!(first_row, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    #[test]
    fn test_first_row_right() {
        let first_row = Direction::Right.first_row(10, 10);
        assert_eq!(first_row, [0, 10, 20, 30, 40, 50, 60, 70, 80, 90])
    }
    #[test]
    fn test_first_row_left() {
        let first_row = Direction::Left.first_row(10, 10);
        assert_eq!(first_row, [9, 19, 29, 39, 49, 59, 69, 79, 89, 99])
    }
}
