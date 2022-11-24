use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

fn main() {
    let file = File::open("./data/13.txt").expect("Could not read data file");
    let reader = BufReader::new(file);

    let data_vec = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;

            let mut segments = line.split_whitespace();

            let name1 = segments.next()?;
            let lose_gain = segments.nth(1)?;
            let abs_score = segments.next()?.parse::<i32>().ok()?;
            let name2 = segments.nth(6)?;
            let name2 = &name2[0..name2.len() - 1];

            let score = if lose_gain == "gain" {
                abs_score
            } else {
                -abs_score
            };

            Some((name1.to_owned(), name2.to_owned(), score))
        })
        .collect::<Vec<_>>();

    let data = data_vec
        .iter()
        .map(|(x, y, z)| ((x.as_str(), y.as_str()), *z))
        .collect::<HashMap<(&str, &str), i32>>();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &HashMap<(&str, &str), i32>) {
    let names = data
        .keys()
        .map(|(name1, _name2)| *name1)
        .unique()
        .collect::<Vec<_>>();

    let total: i32 = names
        .iter()
        .permutations(names.len())
        .map(|x| {
            let total: i32 = x
                .windows(2)
                .map(|names| {
                    let name1 = names[0];
                    let name2 = names[1];
                    let score1 = data
                        .get(&(names[0], names[1]))
                        .unwrap_or_else(|| panic!("Missing data {name1} {name2}"));
                    let score2 = data
                        .get(&(names[1], names[0]))
                        .unwrap_or_else(|| panic!("Missing data {name2} {name1}"));
                    score1 + score2
                })
                .sum();

            let boundary = data.get(&(*x[0], *x[x.len() - 1])).expect("Missing data");
            let boundary_rev = data.get(&(*x[x.len() - 1], *x[0])).expect("Missing data");

            total + boundary + boundary_rev
        })
        .max()
        .expect("Could not find max");

    println!("Part one: {total}");
}

fn part_two(data: &HashMap<(&str, &str), i32>) {
    let names = data
        .keys()
        .map(|(name1, _name2)| *name1)
        .unique()
        .collect::<Vec<_>>();

    let total: i32 = names
        .iter()
        .permutations(names.len())
        .map(|mut x| {
            x.push(x[0]);

            let combinations = x.windows(2).map(|names| {
                let name1 = names[0];
                let name2 = names[1];
                let score1 = data
                    .get(&(names[0], names[1]))
                    .unwrap_or_else(|| panic!("Missing data {name1} {name2}"));
                let score2 = data
                    .get(&(names[1], names[0]))
                    .unwrap_or_else(|| panic!("Missing data {name2} {name1}"));
                score1 + score2
            });

            combinations.clone().sum::<i32>() - combinations.min().expect("No worst pair found")
        })
        .max()
        .expect("Could not find max");

    println!("Part two: {total}");
}
