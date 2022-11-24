use std::error::Error;

const BOARD_SIZE: usize = 5;

fn main() {
    let (numbers, mut boards) = parse_file("./04-input.txt").unwrap();
    if let Some((winning_number, winning_score)) = play_until_winner(&numbers, &mut boards) {
        println!("Win! Number: {} Sum: {}", winning_number, winning_score);
    } else {
        println!("No winner");
    }
}

fn play_until_winner(numbers: &[u32], boards: &mut Vec<Board>) -> Option<(u32, u32)> {
    for number in numbers {
        for board in boards.iter_mut() {
            board.data.iter_mut().for_each(|x| {
                if *x == Some(*number) {
                    *x = None
                }
            });

            if board.is_winner() {
                let winning_number = *number;
                let winning_score = *number * board.data.iter().filter(|x| x.is_some()).fold(0, |acc, x| x.unwrap() + acc);
                return Some((winning_number, winning_score));
            }
        }
    };
    None
}

fn parse_file(filename: &str) -> Result<(Vec<u32>, Vec<Board>), Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    let mut lines = file.lines().peekable();
    let mut boards: Vec<Board> = Vec::new();

    let numbers: Vec<u32> = lines.next().ok_or("File is empty")?
        .split(",")
        .map(|x| x.parse::<u32>().expect("Could not parse to int"))
        .collect();

    lines.next();

    while lines.peek().is_some() {
        let width = lines.peek().unwrap().split_whitespace().count();
        let data: Vec<Option<u32>> = lines.by_ref().take_while(|&x| { x != "" }).map(|line| {
            line.split_whitespace()
                .map(|x| Some(x.parse::<u32>().expect("Could not parse to int")))
        }).flatten().collect();

        let height = data.len()/width;

        boards.push(Board {
            data,
            width,
            height
        });
    }

    Ok((numbers, boards))
}

#[derive(Clone, Debug)]
struct Board {
    data: Vec<Option<u32>>,
    width: usize,
    height: usize,
}

impl Board {
    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn is_winner(&self) -> bool {
        return self.has_row() || self.has_column()
    }

    fn has_row(&self) -> bool {
        (0..self.height).any(|y| {
            (0..self.width).all(|x|{
                self.data[self.index(x, y)] == None
            })
        })
    }

    fn has_column(&self) -> bool {
        (0..self.width).any(|x| {
            (0..self.height).all(|y| {
                self.data[self.index(x, y)] == None
            })
        })
    }
}