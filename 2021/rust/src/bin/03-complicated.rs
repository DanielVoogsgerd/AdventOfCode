fn main() {
    println!("{}", puzzle());
}

fn puzzle() -> u64 {
    let file = std::fs::read_to_string("./03-input.txt").expect("File not found");

    // Interpret input as 2D Vector of booleans
    let input: Vec<Vec<bool>> = file.lines().map(|line| {
        line.chars().map(|char| { char == '1' }).collect()
    }).collect();

    let oxygen = find_gas(&input, |total| {total >= 0});
    let carbon = find_gas(&input, |total| {total < 0});

    let oxygen_count = vec_bool_to_binary(&oxygen);
    let carbon_count = vec_bool_to_binary(&carbon);

    oxygen_count * carbon_count
}

fn vec_bool_to_binary(binary: &Vec<bool>) -> u64 {
    binary.iter().fold(0, |acc, &x|acc*2 + if x {1} else {0})
}

fn find_gas(input: &Vec<Vec<bool>>, gas_identifier: fn(i32) -> bool) -> Vec<bool> {
    let width = input[0].len();
    let mut gas: Vec<bool> = Vec::new();
    let mut filter_codes = input.clone();
    for i in 0..width {
        filter_codes.retain(|bit_string| {
            bit_string.iter().zip(gas.iter()).all(|(x, y)| {x == y})
        });

        if filter_codes.len() == 1{
            gas = filter_codes[0].clone()
        } else if filter_codes.len() > 0 {
            // Mapping 1 to 1 and 0 to -1. The sum should be positive if there are more ones than zeroes.
            let total: i32 = filter_codes.iter().map(|bit_string| {
                if *bit_string.iter().nth(i).expect("Line too short") { 1 } else { -1 }
            }).sum();
            gas.push(gas_identifier(total));
        }
    }

    gas
}

#[cfg(test)]
mod tests {
    use crate::puzzle;

    #[test]
    fn test_answer() {
        assert_eq!(puzzle(), 1800151)
    }
}