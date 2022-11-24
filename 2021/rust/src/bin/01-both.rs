use std::collections::VecDeque;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Could not read file");
    let numbers: Vec<u32> = file.lines().map(|x|x.parse().expect("Could not parse integer")).collect();

    let greater = count_greater(&numbers, 1);
    println!("Answer part 1: {}", greater);

    let greater = count_greater(&numbers, 3);
    println!("Answer part 2: {}", greater);
}

fn count_greater(numbers: &[u32], window: usize) -> usize {
    let greater =
        numbers.iter().skip(window)
        .zip(numbers.iter())
        .filter(|(&x, &y)| x > y)
        .count();

    greater
}