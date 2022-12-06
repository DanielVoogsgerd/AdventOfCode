use std::fs::read_to_string;
use std::hash::Hash;

use itertools::Itertools;

fn main() {
    part_one();
    part_two();
}

fn get_unique_sequence_index<T>(sequence: &[T], win_len: usize) -> Option<usize>
where
    T: Hash + Eq,
{
    Some(
        sequence
            .windows(win_len)
            .position(|window| window.iter().unique().count() == win_len)?
            + win_len,
    )
}

fn part_one() {
    let data = read_to_string("./data/06.txt").expect("Could not read datafile");
    let mut lines = data.lines();
    let line = lines.next().unwrap().chars().collect::<Vec<_>>();

    println!("Part one: {}", get_unique_sequence_index(&line, 4).unwrap())
}

fn part_two() {
    let data = read_to_string("./data/06.txt").expect("Could not read datafile");
    let mut lines = data.lines();
    let line = lines.next().unwrap().chars().collect::<Vec<_>>();

    println!(
        "Part two: {}",
        get_unique_sequence_index(&line, 14).unwrap()
    )
}
