use std::{collections::HashSet, fs::read_to_string};

type Vector = (isize, isize);
type Enumerated<T> = (usize, T);

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("data/03.txt").expect("Could not read datafile");
    let mut coords = HashSet::new();
    let instructions = data
        .lines()
        .flat_map(|line| {
            line.chars().map(|chr| match chr {
                '^' => (0, 1),
                'v' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 0),
            })
        });

    let mut current_coord = (0, 0);
    coords.insert(current_coord);
    for instruction in instructions {
        current_coord = (
            current_coord.0 + instruction.0,
            current_coord.1 + instruction.1,
        );
        coords.insert(current_coord);
    }

    println!("{}", coords.len());
}

fn part_two() {
    let data = read_to_string("data/03.txt").expect("Could not read datafile");
    let mut coords = HashSet::new();
    let instructions: (Vec<Enumerated<Vector>>, Vec<Enumerated<Vector>>) = data
        .lines()
        .flat_map(|line| {
            line.chars().map(|chr| match chr {
                '^' => (0, 1),
                'v' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 0),
            })
        })
        .enumerate()
        .partition(|(i, _item)| i % 2 == 0);

    let mut santa_coord = (0, 0);
    coords.insert(santa_coord);
    for (_i, instruction) in instructions.0 {
        santa_coord = (santa_coord.0 + instruction.0, santa_coord.1 + instruction.1);
        coords.insert(santa_coord);
    }

    let mut robo_coord = (0, 0);
    coords.insert(robo_coord);
    for (_i, instruction) in instructions.1 {
        robo_coord = (robo_coord.0 + instruction.0, robo_coord.1 + instruction.1);
        coords.insert(robo_coord);
    }

    println!("{}", coords.len());
}
