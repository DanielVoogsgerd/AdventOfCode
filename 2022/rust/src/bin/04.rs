use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./data/04.txt").expect("Could not read datafile");

    let ranges: Vec<(u128, u128)> = data
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(range_str1, range_str2)| {
            Some((
                range_to_mask(range_str1).ok()?,
                range_to_mask(range_str2).ok()?,
            ))
        }).collect();

    let supersets = ranges.iter()
        .filter(|(range1, range2)| {
            let union = range1 | range2;
            union == *range1 || union == *range2
        })
        .count();

    println!("{supersets}");

    let intersections = ranges.iter()
        .filter(|(range1, range2)| {
	    *range1 & *range2 != 0
        })
        .count();
    println!("{intersections}");
}

fn range_to_mask(range: &str) -> Result<u128, Box<dyn std::error::Error>> {
    let (lower_bound, upper_bound) = range.split_once('-').ok_or("Could not find range")?;
    let (lower_bound, upper_bound) = (lower_bound.parse::<usize>()?, upper_bound.parse::<usize>()?);

    let mask = 0u128;

    Ok((lower_bound..=upper_bound).fold(mask, |mask, order| mask | 1 << order))
}
