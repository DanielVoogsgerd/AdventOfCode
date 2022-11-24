use std::{collections::HashMap, error::Error, convert::TryInto};

fn main() {
    let (start_polymer, char_map) = parse_file("./14-input.txt").unwrap();

    let grow_map: HashMap<[char; 2], [[char; 2]; 2]> = char_map.iter().map(|(start, &middle) | {
        (*start, [[start[0], middle], [middle, start[1]]])
    }).collect();

    let letter_map = grow_polymer(&start_polymer, 10, &grow_map);
    let answer = letter_map.values().max().unwrap() - letter_map.values().min().unwrap();
    println!("answer part 1: {}", answer);

    let letter_map = grow_polymer(&start_polymer, 40, &grow_map);
    let answer = letter_map.values().max().unwrap() - letter_map.values().min().unwrap();
    println!("answer part 2: {}", answer);
}

fn grow_polymer(start_polymer: &[char], iterations: usize, grow_map: &HashMap<[char; 2], [[char; 2]; 2]>) -> HashMap<char, u64> {
    let mut chunk_count: HashMap<[char; 2], u64> = HashMap::new();
    
    for chunk in start_polymer.windows(2) {
        let chunk_entry = chunk_count.entry(chunk.try_into().unwrap()).or_insert(0);
        *chunk_entry += 1;
    }
    
    chunk_count = replicate(iterations, chunk_count, &grow_map);

    let mut letter_map: HashMap<char, u64> = HashMap::new();

    for (chunk, count) in chunk_count {
        *letter_map.entry(chunk[0]).or_insert(0) += count;
        *letter_map.entry(chunk[1]).or_insert(0) += count;
    }

    *letter_map.entry(start_polymer[0]).or_insert(0) += 1;
    *letter_map.entry(start_polymer[start_polymer.len()-1]).or_insert(0) += 1;

    letter_map.iter_mut().for_each(|(_character, count)| {
        *count /= 2;
    });

    letter_map
}

fn replicate(iterations: usize, start_chunk_count: HashMap<[char; 2], u64>, grow_map: &HashMap<[char; 2], [[char; 2]; 2]>) -> HashMap<[char; 2], u64> {
    let mut chunk_count = start_chunk_count;
    for _ in 0..iterations {
        let mut new_chunk_count: HashMap<[char; 2], u64> = HashMap::new();
        for (&old_chunk, count) in chunk_count.iter() {
            for chunk in grow_map[&old_chunk].iter() {
                let entry = new_chunk_count.entry(*chunk).or_insert(0);
                *entry += count;
            }
        }

        chunk_count = new_chunk_count;
    }

    return chunk_count;
}

fn parse_file(filename: &str) -> Result<(Vec<char>, HashMap<[char; 2], char>), Box<dyn Error>> {
    let data = std::fs::read_to_string(filename)?;
    let mut lines = data.lines();

    let start_polymer_data = lines.next().ok_or("No start polymer found in data file")?;
    let start_polymer: Vec<char> = start_polymer_data.chars().collect();

    lines.next().ok_or("No newline between polymer and mapping")?;

    let char_map: HashMap<[char; 2], char> = lines.map(|line| {
        let mut map_parts = line.split(" -> ");
        let lhs: [char; 2] = map_parts.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap();
        let rhs: char = map_parts.next().unwrap().chars().next().unwrap();
        (lhs, rhs)
    }).collect();

    Ok((start_polymer, char_map))
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    #[test]
    fn test_example_first_iteration() {
        let (start_polymer, char_map) = parse_file("./14-example-input.txt").unwrap();

    }

    #[test]
    fn test_example_tenth_iteration() {

    }
}