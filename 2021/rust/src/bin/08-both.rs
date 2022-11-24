use std::collections::{HashSet};
use std::convert::TryInto;

fn main() {
    let file = std::fs::read_to_string("./08-input.txt").expect("File not found");
    let mut unique_count = 0;
    let mut total_count = 0;

    for line in file.lines() {
        let mut digit_iterator = line.split("|").map(|x| {
            x.split_whitespace().map(|y| {
                y.chars().collect()
            }).collect()
        });

        let left_side_digits = digit_iterator.next().expect("Could not split line");
        let right_side_digits = digit_iterator.next().expect("Could not split line");

        let left_digit_buckets = find_unique_digits(&left_side_digits);
        let right_digit_buckets = find_unique_digits(&right_side_digits);

        let number_map = find_number_mappings(&left_digit_buckets).expect("Error in number mapping");

        unique_count += right_digit_buckets[2].len() + right_digit_buckets[3].len()
            + right_digit_buckets[4].len() + right_digit_buckets[7].len();

        let digits_iter = right_side_digits.iter().map(|digit| {
            number_map.iter().position(|&x| x == digit).unwrap() as u8
        });

        let count: u32 = digits_iter.fold(0, |x, y| {x * 10 + y as u32});
        total_count += count;
    }

    println!("Result part 1: {}", unique_count);
    println!("Result part 2: {}", total_count);
}

fn find_number_mappings<'a>(sample: &[Vec<&'a HashSet<char>>; 8]) -> Result<[&'a HashSet<char>; 10], &'static str> {
    let mut number_map: [Option<&HashSet<char>>; 10] = [None; 10];
    number_map[1] = Some(sample[2][0]);
    number_map[7] = Some(sample[3][0]);
    number_map[4] = Some(sample[4][0]);
    number_map[8] = Some(sample[7][0]);

    number_map[6] = Some(sample[6].iter().filter(|x| {
        return !number_map[7].unwrap().is_subset(x)
    }).nth(0).ok_or("Six set not found")?);

    number_map[9] = Some(sample[6].iter().filter(|x| {
        return number_map[4].unwrap().is_subset(x)
    }).nth(0).ok_or("Nine set not found")?);

    number_map[0] = Some(sample[6].iter().filter(|x| {
        return **x != number_map[6].unwrap() && **x != number_map[9].unwrap()
    }).nth(0).ok_or("Zero set not found")?);

    number_map[3] = Some(sample[5].iter().filter(|x| {
        return number_map[1].unwrap().is_subset(x)
    }).nth(0).ok_or("Three set not found")?);

    number_map[5] = Some(sample[5].iter().filter(|x| {
        return x.is_subset((number_map[6]).unwrap())
    }).nth(0).ok_or("Five set not found")?);

    number_map[2] = Some(sample[5].iter().filter(|x| {
        return **x != number_map[3].unwrap() && **x != number_map[5].unwrap()
    }).nth(0).ok_or("Two set not found")?);

    return Ok(number_map.iter().map(|x| x.unwrap()).collect::<Vec<&HashSet<char>>>().as_slice().try_into().unwrap());
}

fn find_unique_digits(sample: &Vec<HashSet<char>>) -> [Vec<&HashSet<char>>; 8] {
    let mut segment_count: [Vec<&HashSet<char>>; 8] = Default::default();
    for word in sample.iter() {
        let length = word.len();
        segment_count[length].push(word);
    }
    return segment_count;
}

#[cfg(test)]
mod tests {
    use crate::find_unique_digits;

    #[test]
    fn count_test() {
        assert_eq!(find_unique_digits(&vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"]), ());
    }
}