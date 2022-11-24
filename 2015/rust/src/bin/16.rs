use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

enum CorrectionType {
    Fewer,
    Equal,
    More,
}

fn main() {
    let file = File::open("./data/16.txt").expect("Could not read datafile");
    // Part one: Allocation might be uncessary, we can only collect the once that match the conditions
    let data = BufReader::new(file)
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|line| {
            let (name_segment, attributes_segment) = line.split_once(": ")?;
            Some((
                String::from(name_segment),
                attributes_segment
                    .split(", ")
                    .filter_map(|attributes| {
                        let attr_segments = attributes.split_once(": ")?;
                        Some((
                            String::from(attr_segments.0),
                            attr_segments.1.parse::<usize>().ok()?,
                        ))
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect::<Vec<_>>();

    // Considering the size of the dataset a linear search with O(n m) complexity could be faster considering the size of the known values list.
    let mut known_values: HashMap<&str, (CorrectionType, usize)> = HashMap::new();
    known_values.insert("children", (CorrectionType::Equal, 3));
    known_values.insert("cats", (CorrectionType::More, 7));
    known_values.insert("samoyeds", (CorrectionType::Equal, 2));
    known_values.insert("pomeranians", (CorrectionType::Fewer, 3));
    known_values.insert("akitas", (CorrectionType::Equal, 0));
    known_values.insert("vizslas", (CorrectionType::Equal, 0));
    known_values.insert("goldfish", (CorrectionType::Fewer, 5));
    known_values.insert("trees", (CorrectionType::More, 3));
    known_values.insert("cars", (CorrectionType::Equal, 2));
    known_values.insert("perfumes", (CorrectionType::Equal, 1));

    println!("Part one");
    data.iter()
        .filter(|(_name, attributes)| {
            attributes.iter().all(|(attr_name, attr_val)| {
                let known_value = known_values
                    .get(attr_name.as_str())
                    .expect("Unknown attribute");
                known_value.1 == *attr_val
            })
        })
        .for_each(|(name, _)| {
            println!("{name}");
        });

    println!("Part two");
    data.iter()
        .filter(|(_, attributes)| {
            attributes.iter().all(|(attr_name, attr_val)| {
                let known_value = known_values
                    .get(attr_name.as_str())
                    .expect("Unknown attribute");
                match known_value.0 {
                    CorrectionType::Fewer => known_value.1 > *attr_val,
                    CorrectionType::Equal => known_value.1 == *attr_val,
                    CorrectionType::More => known_value.1 < *attr_val,
                }
            })
        })
        .for_each(|(name, _)| {
            println!("{name}");
        });
}
