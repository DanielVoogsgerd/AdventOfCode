use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    let file = File::open("./data/24.txt").expect("Could not open data file");

    let packages = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let total_weight = packages.iter().sum::<usize>();

    let required_group_weight = total_weight / 3;

    for i in 1..10 {
        let combination = packages.iter().combinations(i).filter(|packages| {
            packages.into_iter().map(|x| *x).sum::<usize>() == required_group_weight
        }).map(|packages| {
	    packages.into_iter().product::<usize>()
	}).min();

	if let Some(qe) = combination {
	    println!("{qe}");
	    break;
	}
    }

    let required_group_weight = total_weight / 4;
    for i in 1..10 {
        let combination = packages.iter().combinations(i).filter(|packages| {
            packages.into_iter().map(|x| *x).sum::<usize>() == required_group_weight
        }).map(|packages| {
	    packages.into_iter().product::<usize>()
	}).min();

	if let Some(qe) = combination {
	    println!("{qe}");
	    break;
	}
    }
}
