use std::{collections::{BTreeSet}, error::Error};

fn main() {
    let mut flash_count = 0;
    let mut board = Board::from_file("./11-input.txt").unwrap();
    for _generation in 0..100 {
        flash_count += board.generation();
    }
    println!("Flash count: {}", flash_count);

    let mut board = Board::from_file("./11-input.txt").unwrap();
    let mut i = 0;
    loop {
        i += 1;
        if board.height * board.width == board.generation() as usize{
            break;
        }
    };
    println!("First synchronised generation: {}", i);
}

pub struct Board {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn from_file(filepath: &str) -> Result<Board, Box<dyn Error>> {
        let file = std::fs::read_to_string(filepath)?;
        let data: Vec<Vec<u8>> = file.lines().map(|line| {
            line.chars().map(|char| {
                char.to_digit(10).expect("Could not parse digit") as u8
            }).collect()
        }).collect();

        let height = data.iter().count();
        let width = (data.iter().next().ok_or("No rows in data")?).len();

        Ok(Board {
            data,
            width,
            height
        })
    }

    pub fn generation(&mut self) -> u32 {
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
        self.data.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, octo) | {
                *octo += 1;
                if *octo > 9 {
                    to_flash.push((x, y));
                }
            });
        });

        let mut flashed: BTreeSet<(usize, usize)> = BTreeSet::new();
        while to_flash.len() > 0 {
            let (x, y) = to_flash.pop().unwrap();
            let neighbours = self.get_neighbours(x, y);
            for (n_x, n_y) in neighbours {
                *self.data.iter_mut().nth(n_y).unwrap().iter_mut().nth(n_x).unwrap() += 1;
                if *self.data.get(n_y).unwrap().get(n_x).unwrap() > 9 {
                    if ! to_flash.contains(&(n_x, n_y)) && ! flashed.contains(&(n_x, n_y)) {
                        to_flash.push((n_x, n_y));
                    }
                }
            }
            flashed.insert((x, y));
        }

        for (x, y) in &flashed {
            *self.data.iter_mut().nth(*y).unwrap().iter_mut().nth(*x).unwrap() = 0;
        }

        flashed.len() as u32
    }


    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let directions: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

        return directions.iter().filter_map(|(x_diff, y_diff)| {
            let new_x = (x as i32)+x_diff;
            let new_y = (y as i32)+y_diff;

            if new_x < 0 || new_x >= self.width as i32 || new_y < 0 || new_y >= self.height as i32 {
                return None
            }

            return Some((new_x as usize, new_y as usize));
        }).collect()
    }

    pub fn to_string(&self) -> String {
        self.data.iter().map(|x| {
            x.iter().map(|digit| {digit.to_string()}).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("\n")
    }
}