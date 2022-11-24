use std::error::Error;

fn main() {
    let ones_count = parse_file("./03-input.txt").unwrap();
    let (gamma_count, epsilon_count) = calc_gamma_epsilon_count(ones_count);
    println!("{}", gamma_count * epsilon_count);
}

fn iter_to_binary<I>(iterable: I) -> u32
where I: IntoIterator<Item=bool>,
{
    iterable.into_iter().fold(0, |acc, x|acc*2 + if x {1} else {0})
}

fn parse_file(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename).expect("File not found");

    let binary_length = file.lines().next().ok_or("No first line in file")?.len();

    let mut ones_count: Vec<i32> = vec![0; binary_length];
    for line in file.lines() {
        for (i, bit) in ones_count.iter_mut().enumerate() {
            *bit += if line.chars().nth(i).ok_or("Line too short")? == '1' { 1 } else { -1 }
        }
    }

    Ok(ones_count)
}

fn calc_gamma_epsilon_count(ones_count: Vec<i32>) -> (u32, u32) {
   let gamma_count = iter_to_binary(ones_count.iter().map(|&x| { x >= 0 } ));
   let epsilon_count = iter_to_binary(ones_count.iter().map(|&x| { x < 0 } ));

    (gamma_count, epsilon_count)
}

#[cfg(test)]
mod tests {
    use crate::{calc_gamma_epsilon_count, parse_file};

    #[test]
    fn test_simple_example_input() {
        let ones_count = parse_file("./03-example-input.txt").unwrap();
        let (gamma, epsilon) = calc_gamma_epsilon_count(ones_count);
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }

    #[test]
    fn test_simple_puzzle_input() {
        let ones_count = parse_file("./03-input.txt").unwrap();
        let (gamma, epsilon) = calc_gamma_epsilon_count(ones_count);
        assert_eq!(4139586, gamma*epsilon);
    }
}