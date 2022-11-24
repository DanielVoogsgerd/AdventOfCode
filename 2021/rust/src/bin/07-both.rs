use std::error::Error;

fn main() {
    let positions = parse_file("./07-input.txt").unwrap();
    let ideal_position = find_optimal_position(&positions);

    println!("{}", ideal_position);
    println!("{}", calculate_fuel_linear(&ideal_position, &positions));

    let ideal_position = find_optimal_position_naive(&positions, calculate_fuel_quadratic);
    println!("{}", ideal_position);
    println!("{}", calculate_fuel_quadratic(&ideal_position, &positions));
}

fn parse_file(filename: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;
    let line = file.lines().nth(0).ok_or("No lines in file")?;

    Ok(line.split(',').map(|x|x.parse::<usize>().expect("Could not parse number")).collect())
}

fn find_optimal_position_naive(start_positons: &Vec<usize>, fuel_func: fn(&usize, &Vec<usize>) -> usize) -> usize {
    let minimum_position = start_positons.iter().min().expect("Could not find minimum");
    let max_position = start_positons.iter().max().expect("Could not find maximum");
    let mut optimal_fuel = std::usize::MAX;
    let mut optimal_position = 0;
    for position in *minimum_position..=*max_position {
        let fuel = fuel_func(&position, start_positons);
        if fuel < optimal_fuel {
            optimal_fuel = fuel;
            optimal_position = position;
        }
    }

    optimal_position
}

fn calculate_fuel_linear(position: &usize, start_positions: &Vec<usize>) -> usize {
    start_positions.iter().fold(0,|fuel, crab_position| {
        fuel + (*position as i32 - *crab_position as i32).abs() as usize
    })
}

fn calculate_fuel_quadratic(position: &usize, start_positions: &Vec<usize>) -> usize {
    start_positions.iter().fold(0,|fuel, crab_position| {
        let distance = (*position as i32 - *crab_position as i32).abs() as usize;
        fuel + distance*(distance+1)/2
    })
}

fn find_optimal_position(start_positions: &Vec<usize>) -> usize {
    let mut sorted_positions = start_positions.clone();
    sorted_positions.sort();
    sorted_positions[sorted_positions.len()/2 as usize]
}

#[cfg(test)]
mod tests {
    use crate::{calculate_fuel_linear, calculate_fuel_quadratic, find_optimal_position, find_optimal_position_naive};

    #[test]
    fn test_find_optimal_position_naive() {
        let start_state = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(find_optimal_position_naive(&start_state, calculate_fuel_linear), 2);
    }

    #[test]
    fn test_find_optimal_position() {
        let start_state = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(find_optimal_position(&start_state), 2);
    }

    #[test]
    fn test_find_optimal_position_naive_quadratic_fuel() {
        let start_state = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(find_optimal_position_naive(&start_state, calculate_fuel_quadratic), 5);
    }
}