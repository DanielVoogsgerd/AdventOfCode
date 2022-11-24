use std::error::Error;

const CHUNK_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

fn main() {
    let results = parse_file("./10-input.txt").unwrap();

    let error_score = results.iter().filter_map(|x| {
        if let LineResult::Error((_expected, actual)) = x {
            Some(match actual {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected character")
            })
        } else {
            None
        }
    }).sum::<u32>();

    let mut completion_scores = results.iter().filter_map(|x| {
        if let LineResult::Incomplete(completion) = x {
            return Some(completion.iter().map(|char| {
                match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("Unexpected character")
                }
            }).rfold(0, |acc, cur: i64| {acc*5+cur}));
        }
        None
    }).collect::<Vec<i64>>();

    completion_scores.sort();

    let completion_score = completion_scores[completion_scores.len()/2];

    println!("Error score: {}", error_score);
    println!("Completion score: {}", completion_score);
}

fn parse_file(filename: &str) -> Result<Vec<LineResult>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    Ok(file.lines().map(|x| parse_line(x)).collect())
}

enum LineResult {
    Ok,
    Incomplete(Vec<char>),
    Error((char, char))
}

fn parse_line(line: &str) -> LineResult {
    let mut opening_chars: Vec<char> = vec!();
    for char in line.chars() {
        if CHUNK_PAIRS.iter().map(|(x, _y)| x).find(|&x| *x == char).is_some() {
            opening_chars.push(char);
        } else if CHUNK_PAIRS.iter().map(|(_x, y)| y).find(|&x| *x == char).is_some() {
            let last_opening_char = opening_chars.last().expect("No opening chars left");
            let expected_closing_char = CHUNK_PAIRS.iter().find_map(|(x, y)| {
                if x == last_opening_char {
                    Some(y)
                } else {
                    None
                }
            }).unwrap();
            // println!("{} {}", last_opening_char, expected_closing_char);
            if *expected_closing_char == char {
                opening_chars.pop();
            } else {
                return LineResult::Error((*expected_closing_char, char));
            }
        } else {
            println!("{}", char);
            panic!("Unexpected character");
        }
    }

    if opening_chars.is_empty() {
        LineResult::Ok
    } else {
        LineResult::Incomplete(opening_chars)
    }
}