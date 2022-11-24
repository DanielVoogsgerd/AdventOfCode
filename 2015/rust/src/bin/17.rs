use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    // Lets try brute force here as it should have about 1e6 values.
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./data/17.txt").expect("Could not open file");
    let containers = BufReader::new(file)
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let eggnog_volume = 150;

    let container_count = (1..=containers.len())
        .flat_map(|container_count| containers.iter().combinations(container_count))
        .map(|containers| containers.into_iter().sum::<usize>())
        .filter(|&container_volume| container_volume == eggnog_volume)
        .count();

    println!("Part one: {container_count}");
}

fn part_two() {
    let file = File::open("./data/17.txt").expect("Could not open file");
    let containers = BufReader::new(file)
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let eggnog_volume = 150;

    let container_count = (1..=containers.len())
        .flat_map(|container_count| containers.iter().combinations(container_count))
        .map(|containers| (containers.len(), containers.into_iter().sum::<usize>()))
        .filter(|(_, container_volume)| *container_volume == eggnog_volume)
        .fold((usize::MAX, 0), |acc, cur| {
            match cur.0.cmp(&acc.0) {
                // Set new smallest container size and set the count to one
                std::cmp::Ordering::Less => (cur.0, 1),
                // Increase the smallest container count
                std::cmp::Ordering::Equal => (acc.0, acc.1 + 1),
                // Do nothing
                std::cmp::Ordering::Greater => acc,
            }
        });

    println!("Part two: {container_count:?}");
}
