use std::collections::BTreeMap;
use std::ops::RangeInclusive;

fn main() {
    let file = std::fs::read_to_string("./05-input.txt").expect("File not found");

    let lines: Vec<((usize, usize), (usize, usize))> = file.lines().map(|input|{
        let sides: Vec<(usize, usize)> = input.split(" -> ").map(|side| {
            let coords: Vec<usize> = side.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            return (coords[0], coords[1]);
        }).collect();

        return (sides[0], sides[1]);
    }).collect();

    let mut coords: BTreeMap<(usize, usize), u16> = BTreeMap::new();

    lines.iter().map(|line| interp(line.0, line.1)).flatten().for_each(|coord| {
        let coord_entry = coords.entry(coord).or_insert(0);
        *coord_entry += 1;
    });

    let amount = coords.values().filter(|x| **x >= 2).count();

    println!("{}", amount)
}

fn interp(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)>{
    if start.0 == end.0 {
        return birange_inclusive(start.1, end.1)
            .map(|y| (start.0, y))
            .collect();
    } else if start.1 == end.1 {
        return birange_inclusive(start.0, end.0)
            .map(|x| (x, start.1))
            .collect();
    } else if (start.1 as i16 - end.1 as i16).abs() == (start.0 as i16 - end.0 as i16).abs() {
        return vec!();
    } else {
        unimplemented!();
    }
}

fn birange_inclusive(from: usize, to: usize) -> RangeInclusive<usize> {
    if from < to {
        return from..=to
    } else {
        return to..=from
    }
}

#[cfg(test)]
mod tests {
    use crate::{birange_inclusive, interp};

    #[test]
    fn interp_test() {
        assert_eq!(interp((1, 1), (1, 3)), vec![(1, 1), (1, 2), (1, 3)]);
        assert_eq!(interp((1, 1), (3, 1)), vec![(1, 1), (2, 1), (3, 1)]);
    }

    #[test]
    fn birange_test() {
        assert_eq!(birange_inclusive(1, 3).collect::<Vec<usize>>(), vec![1, 2, 3]);
        assert_eq!(birange_inclusive(1, 1).collect::<Vec<usize>>(), vec![1]);
        assert_eq!(birange_inclusive(3, 1).collect::<Vec<usize>>(), vec![1, 2, 3]);
    }

}