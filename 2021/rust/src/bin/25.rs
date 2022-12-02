use std::{
    fmt::Display,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

// TOOD: Check if using an option with a reference would be faster, but I'm pretty sure it won't be
#[derive(Clone, PartialEq)]
enum Cucumber {
    Empty,
    Right,
    Down,
}

struct Sea {
    width: usize,
    height: usize,
    data: Vec<Cucumber>,
}

impl Sea {
    fn new(data: Vec<Cucumber>, width: usize) -> Self {
        Self {
            height: data.len() / width,
            data,
            width,
        }
    }

    fn next(&mut self) -> bool {
        let width = self.width;
        let height = self.height;
	let mut modified = false;

        for y in 0..height {
            let mut cur_index;
            let mut next_index = y * width;
            let mut cur;
            let mut next = self.data[next_index].clone();
	    let first_empty = next == Cucumber::Empty;
            for x in 0..width-1 {
                cur_index = next_index;
                next_index += 1;

                cur = next;
                next = self.data[next_index].clone();

                if cur == Cucumber::Right && next == Cucumber::Empty {
		    modified = true;
                    self.data[cur_index] = Cucumber::Empty;
                    self.data[next_index] = cur;
                }
            }

	    // Next is more like cur would be inside the loop, because we don't propagate anymore
	    if first_empty && next == Cucumber::Right {
		modified = true;
		self.data[next_index] = Cucumber::Empty;
		self.data[y*width] = Cucumber::Right;
	    }
        }

        for x in 0..width {
            let mut cur_index;
            let mut next_index = x;
            let mut cur;
            let mut next = self.data[next_index].clone();
	    let first_empty = next == Cucumber::Empty;
            for y in 0..height-1 {
                cur_index = next_index;
                next_index += width;

                cur = next;
                next = self.data[next_index].clone();

                if cur == Cucumber::Down && next == Cucumber::Empty {
		    modified = true;
                    self.data[cur_index] = Cucumber::Empty;
                    self.data[next_index] = cur;
                }
            }

	    // Next is more like cur would be inside the loop, because we don't propagate anymore
	    if first_empty && next == Cucumber::Down {
		modified = true;
		self.data[next_index] = Cucumber::Empty;
		self.data[x] = Cucumber::Down;
	    }
        }
	modified
    }
}

impl Display for Sea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                data.push(match self.data[y * self.width + x] {
                    Cucumber::Empty => '.',
                    Cucumber::Right => '>',
                    Cucumber::Down => 'v',
                });
            }
	    data.push('\n');
        }

        writeln!(f, "{}", data)
    }
}

fn main() {
    let data = read_to_string("./data/25.txt").expect("Could not read data file");
    let width = data
        .lines()
        .next()
        .expect("File has no data")
        .len();

    let data = data
        .lines()
        .flat_map(|line| line.chars())
        .map(|chr| match chr {
            '>' => Cucumber::Right,
            'v' => Cucumber::Down,
            '.' => Cucumber::Empty,
            _ => panic!("Invalid direction {}", chr),
        })
        .collect::<Vec<_>>();

    let mut sea = Sea::new(data, width);
    let mut i = 0;
    let last_iter = loop {
	// println!("After {i} steps:");
	// println!("{}", sea);
	i += 1;
	if !sea.next() {
	    break i;
	}
    };
    println!("{}", last_iter);
}
