use std::{fs::read_to_string, collections::BinaryHeap};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("./data/01.txt").expect("Could not read data file");

    let mut line_iter = data.lines();

    let mut max = 0;
    let max = loop {
        let group = line_iter.by_ref().take_while(|line| !(&line).is_empty());


        let sum = group
            .filter_map(|line| line.parse::<usize>().ok())
            .sum::<usize>();

	max = usize::max(sum, max);

	if sum == 0 {
            break max;
        }
    };

    println!("{max}");
}

fn part_two() {
    let data = read_to_string("./data/01.txt").expect("Could not read data file");

    let mut line_iter = data.lines();

    let mut calories_per_elve = BinaryHeap::new();
    
    loop {
        let group = line_iter.by_ref().take_while(|line| !(&line).is_empty());


        let sum = group
            .filter_map(|line| line.parse::<usize>().ok())
            .sum::<usize>();

	if sum == 0 {
            break;
        }

	calories_per_elve.push(sum);
    };

    let answer = (0..3usize).map(|i| {
	calories_per_elve.pop().expect("Not enough data")
    }).sum::<usize>();

    println!("{answer}");
}
