use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{iproduct, Itertools};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./data/19.txt").expect("Could not open datafile");
    let mut data = BufReader::new(file).lines().filter_map(|line| line.ok());

    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

    data.by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (left, right) = line.split_once(" => ")?;
            Some((left.to_owned(), right.to_owned()))
        })
        .for_each(|(from, to)| {
            replacements
                .entry(from)
                .or_insert_with(|| Vec::new())
                .push(to);
        });

    let replacements: Vec<(String, Vec<String>)> = replacements
        .into_iter()
        .sorted_by_key(|(left, _)| left.len())
        .collect();

    let start_molecule = data.next().expect("Could not find start molecule");

    let possible_replacements = find_possible_replacements(&start_molecule, &replacements);

    println!(
        "Part one: {}",
        possible_replacements.iter().unique().count()
    );
}

// I'm not happy with part two yet, part two takes the assumption that
// if any path is found it is the shortest one.
// Furthermore, it finds a result because the search order is defined as depth first,
// shortest replacement key first, this causes us to quite quicky reach the right tree-branches,
// however this is a mere property of this example, and not a general solution to this problem.
// Searching in a different order will take an insane amount of time.

fn part_two() {
    let file = File::open("./data/19.txt").expect("Could not open datafile");
    let mut data = BufReader::new(file).lines().filter_map(|line| line.ok());

    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

    data.by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            // Inverse, because we're working backwards
            // Working backwards has a trivial benefit that replacements become impossible after a while.
            // Searching forward needs to be limited with something like a length check,
            // otherwise you could keep replacing after the length of the molecule has exceeded,
            // which are infinite and incorrect branches.
            let (right, left) = line.split_once(" => ")?;
            Some((left.to_owned(), right.to_owned()))
        })
        .for_each(|(from, to)| {
            replacements
                .entry(from)
                .or_insert_with(|| Vec::new())
                .push(to);
        });

    let replacements: Vec<(String, Vec<String>)> = replacements
        .into_iter()
        .sorted_by_key(|(left, _)| left.len())
	// Must sort from small to large
        // .rev()
        .collect();

    // print!("{replacements:#?}");

    let start_molecule = data.next().expect("Could not find start molecule");

    // Let's do a depth first search
    let mut search_queue: Vec<(usize, String)> = Vec::new();

    search_queue.push((0, start_molecule));

    // Should not be necessary
    let mut visited: HashSet<String> = HashSet::new();

    let min_depth = 'l: loop {
        let new_items = if let Some((depth, current_molecule)) = search_queue.pop() {
            find_possible_replacements(&current_molecule, &replacements)
                .into_iter()
                .map(move |new_molecule| (depth + 1, new_molecule))
        } else {
            break None;
        };

        for (depth, new_molecule) in new_items {
            if new_molecule == "e" {
                break 'l Some(depth);
            }
            if !visited.contains(&new_molecule) {
                visited.insert(new_molecule.clone());
                search_queue.push((depth, new_molecule));
            }
        }
    };

    println!("Part two: {}", min_depth.expect("Could not find result"));
    println!("Created {} molecules", visited.len());
}

fn find_possible_replacements(input: &str, replacements: &[(String, Vec<String>)]) -> Vec<String> {
    replacements
        .iter()
        .flat_map(|(from, tos)| {
            let replacement_indices = input.match_indices(from);
            let combinations = iproduct!(replacement_indices, tos.iter());

            combinations
        })
        .filter_map(|((index, from), to)| {
            Some(String::from_iter(
                [
                    input.get(0..index)?,
                    to.as_str(),
                    input.get((index + from.len())..)?,
                ]
                .into_iter(),
            ))
        })
        .collect::<Vec<_>>()
}
