use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let sum = read_to_string("./data/03.txt")
        .expect("Could not read data file")
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let part1 = iter.by_ref().take(line.len() / 2).collect::<HashSet<_>>();
            let part2 = iter.collect::<HashSet<_>>();

            let mut intersection = part1.intersection(&part2);
            *intersection.next().expect("Could not find overlap")
        })
        .map(|chr| {
            let byte = chr as u8;
            if byte >= b'a' {
                (byte - b'a' + 1) as u32
            } else {
                (byte - b'A' + 27) as u32
            }
        })
        .sum::<u32>();

    println!("{sum}")
}

fn part_two() {
    let sum = read_to_string("./data/03.txt")
        .expect("Could not read data file")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| -> char {
            chunk
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|l, r| l.intersection(&r).copied().collect::<HashSet<_>>())
                .expect("Could not find overlap")
                .into_iter()
                .next()
                .expect("Could not find overlap")
        })
        .map(|chr| {
            let byte = chr as u8;
            (if byte >= b'a' {
                byte - b'a' + 1
            } else {
                byte - b'A' + 27
            }) as u32
        })
        .sum::<u32>();

    println!("{sum}")
}
