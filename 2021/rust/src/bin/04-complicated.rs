const BOARD_SIZE: usize = 5;

fn main() {
    let file = std::fs::read_to_string("./04-input.txt").expect("File not found");
    let mut lines = file.lines();
    let mut boards: Vec<Vec<Option<u32>>> = Vec::new();
    let mut board: Vec<Option<u32>> = Vec::new();

    let numbers: Vec<u32> = lines.next().expect("File is empty")
        .split(",")
        .map(|x| x.parse::<u32>().expect("Could not parse to int"))
        .collect();
    lines.next();

    for line in lines {
        if line.eq("") {
            boards.push(board);
            board = Vec::new();
            continue;
        }

        let mut line_numbers: Vec<Option<u32>> = line.split_whitespace()
            .map(|x| Some(x.parse::<u32>().expect("Could not parse to int")))
            .collect();

        board.append(&mut line_numbers);
    }
    boards.push(board);

    let mut winning_number: Option<u32> = None;
    let mut winning_score: Option<u32> = None;

    for number in &numbers {
        let mut i = 0;
        let mut winners = vec!();
        for board in &mut boards {
            board.iter_mut().for_each(|x| {
                if *x == Some(*number) {
                    *x = None
                }
            });

            if is_winner(&board.clone()) {
                winners.push(i.clone());
            }

            i+=1;
        }
        if boards.len()-winners.len() == 0 {
            winning_score = Some(boards[0].iter().fold(0, |acc, x| acc + x.unwrap_or(0)));
            winning_number = Some(number.clone());
            break;
        }
        winners.sort();
        winners.reverse();
        for i in winners {
            boards.remove(i);
        }
    }
    let winning_number = winning_number.unwrap();

    println!("Win! Number:{} Sum: {} Product: {}", winning_number, winning_score.unwrap(), winning_number * winning_score.unwrap());
}

fn is_winner(board: &Vec<Option<u32>>) -> bool {
    return has_row(board) || has_column(board)
}

fn has_row(board: &Vec<Option<u32>>) -> bool {
    for y in 0..BOARD_SIZE {
        let mut found = true;
        for x in 0..BOARD_SIZE {
            let index: usize = BOARD_SIZE * y + x;
            if board[index] != None {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }

    return false;
}

fn has_column(board: &Vec<Option<u32>>) -> bool {
    for x in 0..BOARD_SIZE {
        let mut found = true;
        for y in 0..BOARD_SIZE {
            let index: usize = BOARD_SIZE * y + x;
            if board[index] != None {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }

    return false;
}