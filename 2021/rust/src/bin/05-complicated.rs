use std::collections::BTreeMap;
use std::error::Error;
use std::ops::RangeInclusive;

fn main() {
    let lines: Vec<((usize, usize), (usize, usize))> = parse_input("./05-input.txt").unwrap();

    let mut coords: BTreeMap<(usize, usize), u16> = BTreeMap::new();
    lines.iter().map(|line| interp(line.0, line.1)).flatten().for_each(|coord| {
        let coord_entry = coords.entry(coord).or_insert(0);
        *coord_entry += 1;
    });

    let amount = coords.values().filter(|&x| *x >= 2).count();

    println!("Result: {}", amount)
}

fn parse_input(filename: &str) -> Result<Vec<((usize, usize), (usize, usize))>, Box<dyn Error>> {
    let file = std::fs::read_to_string(filename)?;

    Ok(file.lines().map(|input|{
        let sides: Vec<(usize, usize)> = input.split(" -> ").map(|side| {
            let coords: Vec<usize> = side.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            (coords[0], coords[1])
        }).collect();

        (sides[0], sides[1])
    }).collect())
}

fn interp(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)>{
    if start.0 == end.0 {
        birange_inclusive(start.1, end.1)
            .map(|y| (start.0, y))
            .collect()
    } else if start.1 == end.1 {
        birange_inclusive(start.0, end.0)
            .map(|x| (x, start.1))
            .collect()
    } else if (start.1 as i16 - end.1 as i16).abs() == (start.0 as i16 - end.0 as i16).abs() {
        let dir:i16 = if (start.0 < end.0) ^ (start.1 < end.1) {-1} else {1};
        let y_start: usize = if start.0 < end.0 { start.1 } else { end.1 };
        birange_inclusive(start.0, end.0).enumerate().map(|(i, x)| {
            (x, (y_start as i16 + dir*i as i16) as usize)
        }).collect()
    } else {
        unimplemented!();
    }
}

fn birange_inclusive(from: usize, to: usize) -> RangeInclusive<usize> {
    if from < to {
        from..=to
    } else {
        to..=from
    }
}

#[cfg(test)]
mod tests {
    use crate::{birange_inclusive, interp};

    #[test]
    fn interp_test() {
        assert_eq!(interp((1, 1), (1, 3)), vec![(1, 1), (1, 2), (1, 3)]);
        assert_eq!(interp((1, 1), (3, 1)), vec![(1, 1), (2, 1), (3, 1)]);

        assert_eq!(interp((1, 1), (3, 3)), vec![(1, 1), (2, 2), (3, 3)]);
        assert_eq!(interp((3, 3), (1, 1)), vec![(1, 1), (2, 2), (3, 3)]);

        assert_eq!(interp((1, 3), (3, 1)), vec![(1, 3), (2, 2), (3, 1)]);
        assert_eq!(interp((3, 1), (1, 3)), vec![(1, 3), (2, 2), (3, 1)]);
    }

    #[test]
    fn birange_test() {
        assert_eq!(birange_inclusive(1, 3).collect::<Vec<usize>>(), vec![1, 2, 3]);
        assert_eq!(birange_inclusive(1, 1).collect::<Vec<usize>>(), vec![1]);
        assert_eq!(birange_inclusive(3, 1).collect::<Vec<usize>>(), vec![1, 2, 3]);
    }
}