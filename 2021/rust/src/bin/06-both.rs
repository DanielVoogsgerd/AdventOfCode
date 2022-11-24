use std::error::Error;

fn main() {
    let fish = parse_file("06-input.txt").unwrap();

    println!("After 80 days: {} fish", predict_lanternfish(&fish, 80));
    println!("After 256 days: {} fish", predict_lanternfish(&fish, 256));
}

fn parse_file(filename: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    let line = file.lines().nth(0).ok_or("No line in file")?;
    Ok(line.split(',').map(|x| x.parse::<usize>().unwrap()).collect())
}

fn predict_lanternfish(start_state: &[usize], days: u16) -> u64 {
    let mut fish = vec![0; 9];
    start_state.iter().for_each(|x| {
        fish[*x] += 1
    });

    for _ in 0..days {
        let new_fish = fish[0];
        for i in 0..8 {
            fish[i] = fish[i+1]
        }

        fish[6] += new_fish;
        fish[8] = new_fish;
    }

    fish.iter().fold(0, |x, acc| x+acc)
}

#[cfg(test)]
mod tests {
    use crate::predict_lanternfish;

    #[test]
    fn latern_test() {
        let start_state = vec![3, 4, 3, 1, 2];
        assert_eq!(predict_lanternfish(&start_state, 18), 26);
        assert_eq!(predict_lanternfish(&start_state, 80), 5934);
    }
}