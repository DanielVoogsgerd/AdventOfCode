use nom::{bytes::complete::tag, character::complete, multi::separated_list1};
use std::{collections::BTreeSet, fs::read_to_string};

use itertools::Itertools;

const MAX_DIM: usize = 3;

fn main() {
    let data = read_to_string("./data/18.txt").expect("Could not read data file");

    let answer = part_one(&data);
    println!("Part one: {answer}");

    let answer = part_two(&data);
    println!("Part two: {answer}");
}

fn part_one(data: &str) -> usize {
    let mut coords = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let mut sides = coords.len() * 6;

    let dims = [0usize, 1, 2];

    for dim in dims {
        let fixed = dims
            .iter()
            .cloned()
            .filter(|&x| x != dim)
            .collect::<Vec<_>>();
        let mut order = fixed.clone();
        order.push(dim);

        coords.sort_by_key(|x| make_sort_key(x, order.clone()));

        sides -= coords
            .iter()
            .tuple_windows()
            .filter(|(a, b)| a[fixed[0]] == b[fixed[0]])
            .filter(|(a, b)| a[fixed[1]] == b[fixed[1]])
            .filter(|(a, b)| a[dim] == b[dim] - 1)
            .count()
            * 2;
    }

    sides
}

fn part_two(data: &str) -> usize {
    let coords = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|x| x.1)
        .collect::<Vec<_>>();

    let min_max = (0..MAX_DIM)
        .map(|dim| {
            (
                coords.iter().map(|coord| coord[dim]).min().unwrap() - 1,
                coords.iter().map(|coord| coord[dim]).max().unwrap() + 1,
            )
        })
        .collect::<Vec<_>>();

    let starting_point = min_max.iter().map(|range| range.0).collect::<Vec<_>>();
    let mut stack = vec![starting_point];

    let mut visited = BTreeSet::new();
    let mut sides = 0;
    while let Some(coord) = stack.pop() {
	let neighbours = get_neighbours(&coord, &min_max);

	for neighbour in neighbours {
	    if coords.contains(&neighbour) {
		sides += 1;
	    } else if !visited.contains(&neighbour) {
		visited.insert(neighbour.clone());
		stack.push(neighbour);
	    }
	}
    }

    sides
}

fn get_neighbours(coord: &[i16], min_max: &[(i16, i16)]) -> Vec<Vec<i16>> {
    let mut neighbours = Vec::with_capacity(2 * MAX_DIM);
    for dimension in 0..MAX_DIM {
	if coord[dimension] > min_max[dimension].0 {
	    let mut neighbour = coord.to_owned();
	    neighbour[dimension] -= 1;
	    neighbours.push(neighbour);
	}
	if coord[dimension] < min_max[dimension].1 {
	    let mut neighbour = coord.to_owned();
	    neighbour[dimension] += 1;
	    neighbours.push(neighbour);
	}
    }

    neighbours
}

fn make_sort_key(coord: &[i16], order: impl IntoIterator<Item = usize>) -> usize {
    let mut it = order.into_iter();
    let mut total = coord[it.next().expect("Need at least one sort key")] as usize;
    for key in it {
        total = (total << 8) + coord[key] as usize;
    }

    total
}

fn parse_line(line: &str) -> nom::IResult<&str, Vec<i16>> {
    separated_list1(tag(","), complete::i16)(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/18-example.txt").expect("Could not load example data file");

        assert_eq!(part_one(&data), 64);
    }
    #[test]
    fn test_part_two() {
        let data =
            read_to_string("./data/18-example.txt").expect("Could not load example data file");

        assert_eq!(part_two(&data), 58);
    }
}
