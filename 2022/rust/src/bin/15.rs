use std::fs::read_to_string;

use itertools::Itertools;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    sequence::{preceded, separated_pair},
    IResult,
};

type Coord = (i64, i64);

struct Sensor {
    position: Coord,
    range: i64,
}

impl Sensor {
    fn in_range(&self, coord: Coord) -> bool {
	manhattan_distance(self.position, coord) <= self.range
    }
}

fn main() {
    let data = read_to_string("./data/15.txt").expect("Could not load data file");
    println!("Part one: {}", part_one(&data, 2_000_000));
    println!("Part two: {}", part_two(&data, (0, 4_000_000), (0, 4_000_000)));
}

fn manhattan_distance(coord1: Coord, coord2: Coord) -> i64 {
    i64::abs(coord1.0 - coord2.0) + i64::abs(coord1.1 - coord2.1)
}

fn part_one(data: &str, row: i64) -> i64 {
    let sensors = data.lines().filter_map(|line| {
        let (sensor, beacon) = parse_line(line).ok()?.1;

        let distance = manhattan_distance(sensor, beacon);

        Some((sensor, distance))
    });

    let mut excluded_ranges = sensors
        .filter_map(|(sensor, excluded_distance)| {
            let row_distance = i64::abs(sensor.1 - row);
            let excluded_radius = excluded_distance - row_distance;

            (excluded_radius >= 0)
                .then_some((sensor.0 - excluded_radius, sensor.0 + excluded_radius))
        })
        .collect::<Vec<_>>();

    excluded_ranges.sort();

    get_non_overlapping_ranges(excluded_ranges)
        .iter()
        .map(|(start, end)| end - start)
        .sum::<i64>()
}

fn part_two(data: &str, x_range: (i64, i64), y_range: (i64, i64)) -> i64 {
    let sensors = data
        .lines()
        .filter_map(|line| {
            let (sensor, beacon) = parse_line(line).ok()?.1;
            let distance = manhattan_distance(sensor, beacon);

            Some(Sensor {
                position: sensor,
                range: distance,
            })
        })
        .collect::<Vec<_>>();

    let pois = get_all_points_of_interest(&sensors);

    let valid_poi = pois.iter().filter(|&poi| {
	is_valid_coord(*poi, x_range, y_range)
    }).filter(|&poi| {
	!sensors.iter().any(|sensor| sensor.in_range(*poi))
    }).next().expect("Could not find valid POI");

    valid_poi.0 * 4_000_000 + valid_poi.1
}

fn is_valid_coord(coord: Coord, x_range: (i64, i64), y_range: (i64, i64)) -> bool {
    x_range.0 <= coord.0 && coord.0 <= x_range.1 && y_range.0 <= coord.1 && coord.1 <= y_range.1
}

fn get_points_of_interest(sensor1: &Sensor, sensor2: &Sensor) -> Option<[(i64, i64); 2]> {
    let distance = manhattan_distance(sensor1.position, sensor2.position);
    let range_sum = sensor1.range + sensor2.range;

    if distance > range_sum {
        return None;
    }

    let distance_vec = (
        sensor2.position.0 - sensor1.position.0,
        sensor2.position.1 - sensor1.position.1,
    );
    let offset = range_sum - distance;

    // Does not lock in a single coordinate, could theoretically lead to missed cases
    if offset % 2 == 1 {
        return None;
    }

    let offset = offset / 2;

    // let horizontal_direction = distance_vec.0 / i64::abs(distance_vec.0);
    let horizontal_direction = match distance_vec.0.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    // let vertical_direction = distance_vec.1 / i64::abs(distance_vec.1);
    let vertical_direction = match distance_vec.1.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    let abs_horizontal_distance = i64::abs(distance_vec.0);
    let abs_vertical_distance = i64::abs(distance_vec.1);

    let pathlen = sensor1.range - offset;

    // First horizontal then vertical
    let horizontal_steps = i64::min(abs_horizontal_distance, pathlen);
    let vertical_steps = i64::max(pathlen - horizontal_steps, 0);

    let closest_x = sensor1.position.0 + horizontal_steps * horizontal_direction;
    let closest_y = sensor1.position.1 + vertical_steps * vertical_direction;

    let poi1 = if vertical_steps > 0 {
        let offset_direction = if horizontal_direction == 0 {
            -1
        } else {
            horizontal_direction
        };
        (closest_x + ((offset + 1) * offset_direction), closest_y)
    } else {
        let offset_direction = if vertical_direction == 0 {
            -1
        } else {
            vertical_direction
        };
        (closest_x, closest_y - ((offset + 1) * offset_direction))
    };

    // First vertical then horizontal
    let vertical_steps = i64::min(abs_vertical_distance, pathlen);
    let horizontal_steps = i64::max(pathlen - vertical_steps, 0);

    let closest_x = sensor1.position.0 + horizontal_steps * horizontal_direction;
    let closest_y = sensor1.position.1 + vertical_steps * vertical_direction;

    let poi2 = if horizontal_steps > 0 {
        let offset_direction = if vertical_direction == 0 {
            1
        } else {
            vertical_direction
        };
        (closest_x, closest_y + ((offset + 1) * offset_direction))
    } else {
        let offset_direction = if horizontal_direction == 0 {
            1
        } else {
            horizontal_direction
        };
        (closest_x - ((offset + 1) * offset_direction), closest_y)
    };

    Some([poi1, poi2])
}

fn get_all_points_of_interest(sensors: &[Sensor]) -> Vec<Coord> {
    // For each sensor check if it locks in space in the corner

    // For each combination of sensor find point of overlap and add them
    sensors
        .iter()
        .tuple_combinations()
        .filter_map(|(sensor1, sensor2)| get_points_of_interest(sensor1, sensor2))
        .flatten()
        .collect()
}

fn parse_line(line: &str) -> IResult<&str, (Coord, Coord)> {
    separated_pair(
        preceded(take_until("x="), parse_coordinate),
        tag(":"),
        preceded(take_until("x="), parse_coordinate),
    )(line)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coord> {
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)
}

/// Precondition: Assumes a sorted iterator as input
fn get_non_overlapping_ranges(
    overlapping_ranges: impl IntoIterator<Item = (i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut non_overlapping_ranges = Vec::new();
    let mut it = overlapping_ranges.into_iter();

    let (mut current_start, mut current_end) = it.next().expect("Received empty iter");

    for (start, end) in it {
        if start > current_end {
            non_overlapping_ranges.push((current_start, current_end));
            current_start = start;
        }

        current_end = i64::max(current_end, end);
    }

    non_overlapping_ranges.push((current_start, current_end));

    non_overlapping_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data =
            read_to_string("./data/15-example.txt").expect("Could not load example data filkke");
        assert_eq!(part_one(&data, 10), 26)
    }

    #[test]
    fn test_part_two() {
        let data =
            read_to_string("./data/15-example.txt").expect("Could not load example data filkke");
        assert_eq!(part_two(&data, (0, 20), (0, 20)), 56_000_011);
    }

    #[test]
    fn test_parse_line() {
        let res = parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15")
            .unwrap()
            .1;
        assert_eq!(res, ((2, 18), (-2, 15)))
    }

    #[test]
    fn test_get_non_overlapping() {
        let res = get_non_overlapping_ranges([(0, 3), (1, 12), (1, 3), (14, 18)]);
        assert_eq!(res, vec![(0, 12), (14, 18)])
    }

    #[test]
    fn test_get_poi() {
        let sensor1 = Sensor {
            position: (6, 8),
            range: 4,
        };
        let sensor2 = Sensor {
            position: (7, 4),
            range: 3,
        };

        let poi = get_points_of_interest(&sensor1, &sensor2).unwrap();
        assert_eq!(poi.len(), 2);
        assert!(poi.contains(&(4, 5)));
        assert!(poi.contains(&(9, 6)));
    }
}
