use std::{collections::HashSet, fs::read_to_string};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("data/05.txt").expect("Could not read datafile");

    let bad_segments = ["ab", "cd", "pq", "xy"];

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let nice = data.lines().filter(|line| {
        for bad_segment in bad_segments {
            if line.contains(bad_segment) {
                return false;
            }
        }

        if line.chars().filter(|chr| vowels.contains(chr)).count() < 3 {
            return false;
        }

        if !line.chars().zip(line.chars().skip(1)).any(|(x, y)| x == y) {
            return false;
        }

        true
    });

    println!("Answer {}", nice.count())
}

fn part_two() {
    let data = read_to_string("data/05.txt").expect("Could not read datafile");

    let nice = data.lines().filter(|line| {
        let l2: Vec<_> = line.chars().collect();

        if !l2.windows(3).into_iter().any(|x| x[0] == x[2]) {
            return false;
        }

        let quintuplets = l2
            .windows(4)
            .into_iter()
            .filter(|x| x[0] == x[1] && x[0] == x[2] && x[0] == x[3])
            .count();
        let triplets = l2
            .windows(3)
            .into_iter()
            .filter(|x| x[0] == x[1] && x[0] == x[2])
            .count();
        let combos = l2.windows(2).collect::<HashSet<_>>().len();

        if combos + triplets - quintuplets == line.len() - 1 {
            return false;
        }

        true
    });

    println!("Answer {}", nice.count())
}
