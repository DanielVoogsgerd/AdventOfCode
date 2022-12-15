use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};

type Coord = (u32, u32);

fn main() {
    let data = read_to_string("./data/14.txt").expect("Could not read data file");
    let answer = part_one(&data);
    println!("Part one: {answer}");
    let answer = part_two(&data);
    println!("Part two: {answer}");
}

#[derive(Debug)]
enum Material {
    Rock,
    Sand,
}

fn part_one(data: &str) -> usize {
    let mut filled: HashMap<Coord, Material> = HashMap::new();
    let paths = data
        .lines()
        .map(|line| parse_path(line).expect("Could not parse path").1)
        .collect::<Vec<_>>();


    let mut max_y = 0;
    for coord in paths.iter().flat_map(|path| coordinates_from_path(&path)) {
	filled.insert(coord, Material::Rock);
	max_y = u32::max(max_y, coord.1);
    }

    let mut sand_grain_count = 0;
    'outer: loop {

	let mut cur_x = 500;
	let mut cur_y = 0;

	loop {
	    if cur_y > max_y {
		break 'outer;
	    }
	    if !filled.contains_key(&(cur_x, cur_y+1)) {
		cur_y += 1;
		continue;
	    }
	    if !filled.contains_key(&(cur_x-1, cur_y+1)) {
		cur_x -= 1;
		cur_y += 1;
		continue;
	    }
	    if !filled.contains_key(&(cur_x+1, cur_y+1)) {
		cur_x += 1;
		cur_y += 1;
		continue;
	    }

	    filled.insert((cur_x, cur_y), Material::Sand);
	    sand_grain_count += 1;
	    break;
	}
    }

    sand_grain_count
}

fn part_two(data: &str) -> usize {
    let mut filled: HashMap<Coord, Material> = HashMap::new();
    let paths = data
        .lines()
        .map(|line| parse_path(line).expect("Could not parse path").1)
        .collect::<Vec<_>>();


    let mut max_y = 0;
    for coord in paths.iter().flat_map(|path| coordinates_from_path(&path)) {
	filled.insert(coord, Material::Rock);
	max_y = u32::max(max_y, coord.1);
    }

    let floor = max_y + 2;

    let mut sand_grain_count = 0;
    'outer: loop {

	let mut cur_x = 500;
	let mut cur_y = 0;

	if filled.contains_key(&(cur_x, cur_y)) {
	    break;
	}

	loop {
	    if cur_y+1 == floor {
		filled.insert((cur_x, cur_y), Material::Sand);
		sand_grain_count += 1;
		break;
	    }

	    if !filled.contains_key(&(cur_x, cur_y+1)) {
		cur_y += 1;
		continue;
	    }
	    if !filled.contains_key(&(cur_x-1, cur_y+1)) {
		cur_x -= 1;
		cur_y += 1;
		continue;
	    }
	    if !filled.contains_key(&(cur_x+1, cur_y+1)) {
		cur_x += 1;
		cur_y += 1;
		continue;
	    }

	    filled.insert((cur_x, cur_y), Material::Sand);
	    sand_grain_count += 1;
	    break;
	}
    }

    sand_grain_count
}

fn parse_path(data: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(data)
}

fn coordinates_from_path<'a>(path: &'a [Coord]) -> impl Iterator<Item = Coord> + 'a {
    path.windows(2).flat_map(|coords| {
        let from = coords[0];
        let to = coords[1];

        let min_x = u32::min(from.0, to.0);
        let max_x = u32::max(from.0, to.0);
        let min_y = u32::min(from.1, to.1);
        let max_y = u32::max(from.1, to.1);

        (min_x..=max_x).cartesian_product(min_y..=max_y)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data =
            read_to_string("./data/14-example.txt").expect("Could not load example datafile");
        let answer = part_one(&data);
        assert_eq!(answer, 24);
    }

    #[test]
    fn test_part_two_example() {
        let data =
            read_to_string("./data/14-example.txt").expect("Could not load example datafile");
        let answer = part_two(&data);
        assert_eq!(answer, 93);
    }

    #[test]
    fn test_parse_path() {
        let path = parse_path("498,4 -> 498,6 -> 496,6").unwrap().1;
        assert_eq!(path, vec![(498, 4), (498, 6), (496, 6)]);
    }
}
