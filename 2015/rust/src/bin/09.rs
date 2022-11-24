use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./data/09.txt").expect("Could not read datafile");

    let info = data
        .lines()
        .filter_map(|line| {
            let mut segments = line.split_whitespace();

            Some((
                segments.next()?,
                segments.nth(1)?,
                segments
                    .nth(1)?
                    .parse::<u32>()
                    .expect("Cost could not be parsed"),
            ))
        })
        .collect::<Vec<_>>();

    let locations = info
        .iter()
        .map(|x| x.0)
        .chain(info.iter().map(|x| x.1))
        .collect::<BTreeSet<_>>();

    let mut distances: BTreeMap<(&str, &str), u32> = BTreeMap::new();

    for (loc1, loc2, cost) in info {
        distances.insert((loc1, loc2), cost);
        distances.insert((loc2, loc1), cost);
    }

    let shortest = locations
        .iter()
        .permutations(locations.len())
        .map(|order| {
            order
                .windows(2)
                .map(|loc_pair| {
                    distances
                        .get(&(*loc_pair[0], *loc_pair[1]))
                        .cloned()
                        .unwrap_or(u32::MAX / (locations.len() as u32))
                })
                .sum::<u32>()
        })
        .min()
        .expect("Could not find a min path");

    let longest = locations
        .iter()
        .permutations(locations.len())
        .map(|order| {
            order
                .windows(2)
                .map(|loc_pair| {
                    distances
                        .get(&(*loc_pair[0], *loc_pair[1]))
                        .cloned()
                        .unwrap_or(0)
                })
                .sum::<u32>()
        })
        .max()
        .expect("Could not find a min path");

    eprintln!("Answer part one: {shortest}");
    eprintln!("Answer part two: {longest}");
}
