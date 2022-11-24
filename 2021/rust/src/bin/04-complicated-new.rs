use std::error::Error;

fn main() {
    let (numbers, mut boards) = parse_file("./04-input.txt").unwrap();

    if let Some((winning_number, winning_score)) = play_until_winner(&numbers, &mut boards) {
        println!("Win! Number:{} Sum: {} Product: {}", winning_number, winning_score, winning_number * winning_score);
    } else {
        println!("Nobody won");
    }
}

fn play_until_winner(numbers: &[u32], boards: &mut Vec<Board>) -> Option<(u32, u32)> {
    for &number in numbers {
        let mut winners = Vec::new();

        for (i, board) in boards.iter_mut().enumerate() {
            for x in board.data.iter_mut() {
                if *x == Some(number) {
                    *x = None
                }
            }

            if board.is_winner() {
                winners.push(i.clone());
            }
        }

        if boards.len()-winners.len() == 0 {
            let winning_score = boards[0].data.iter().filter_map(|&x| {x}).sum();
            let winning_number = number;
            return Some((winning_number, winning_score));
        }

        winners.sort();
        winners.reverse();
        for i in winners {
            boards.remove(i);
        }
    }

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