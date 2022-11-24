#![feature(test)]
extern crate test;

use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("data/01.txt").expect("Could not load data file");

    let res = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| if chr == '(' { 1i32 } else { -1i32 })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Answer part {}", res);
}

fn part_two() {
    let data = read_to_string("data/01.txt").expect("Could not load data file");

    let mapped = data
        .lines()
        .flat_map(|line| line.chars().map(|chr| if chr == '(' { 1 } else { -1 }));

    let mut total = 0i32;
    let mut iter = 0u32;
    for token in mapped {
        if total < 0 {
            break;
        }

        total += token;
        iter += 1;
    }

    println!("{}", iter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
