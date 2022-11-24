#![feature(test)]
extern crate test;

fn main() {
    let input = 36_000_000;
    println!("Answer part one: {}", first_house_past_fast(input));
    println!("Answer part two: {}", first_house_past_fast_max_presents(input));
}

fn first_house_past(min_sum: usize) -> usize {
    // let mut i = get_lower_limit(n as f32);
    for house_number in 1.. {
	let mut sum = 0;

	for divisor_candidate in 1.. {
	    if divisor_candidate * divisor_candidate > house_number {
		break;
	    }

	    if house_number % divisor_candidate == 0 {
		sum += divisor_candidate;
		let inversion = house_number / divisor_candidate;
		if inversion != divisor_candidate {
		    sum += inversion;
		}
	    }
	}

        if sum * 10 >= min_sum {
            return house_number;
        }
    }

    unreachable!();
}

fn first_house_past_fast(min_sum: usize) -> usize {
    let mut presents = vec![0; min_sum/10];
    for elf in 1..min_sum/10 {
	for i in 1.. {
	    let house_num = i * elf;
	    if house_num >= min_sum/10 {
		break;
	    }
	    presents[house_num] += elf*10;
	}
    }
    presents.iter().enumerate().find(|(_house_number, presents)| **presents > min_sum).expect("Did not find answer").0
}

fn first_house_past_fast_max_presents(min_sum: usize) -> usize {
    let mut presents = vec![0; min_sum/11];
    for elf in 1..min_sum/11 {
	for i in 1..=50 {
	    let house_num = i * elf;
	    if house_num >= min_sum/11 {
		break;
	    }
	    presents[house_num] += elf*11;
	}
    }
    presents.iter().enumerate().find(|(_house_number, presents)| **presents > min_sum).expect("Did not find answer").0
}

fn first_house_past_complex(min_sum: usize) -> usize {
    let mut presents_left = [50; 1_000_000];

    for house_number in 1.. {
	let mut sum = 0;

        for divisor_candidate in 1.. {
            if divisor_candidate * divisor_candidate > house_number {
                break;
            }

            if house_number % divisor_candidate == 0 {
		if presents_left[divisor_candidate] > 0 {
		    sum += divisor_candidate;
		    presents_left[divisor_candidate] -= 1;
		}
                let inversion = house_number / divisor_candidate;
                if inversion != divisor_candidate {
		    if presents_left[inversion] > 0 {
			sum += inversion;
			presents_left[inversion] -= 1;
		    }
                }
            }
        }

	if sum * 11 >= min_sum {
	    return house_number;
	}
    }

    unreachable!();
}

fn sum_of_divisors_fast(n: usize) -> usize {
    let mut sum = 0;

    for divisor_candidate in 1.. {
        if divisor_candidate * divisor_candidate > n {
            break;
        }

        if n % divisor_candidate == 0 {
            sum += divisor_candidate;
            let inversion = n / divisor_candidate;
            if inversion != divisor_candidate {
                sum += inversion;
            }
        }
    }

    sum
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| first_house_past_fast(36_000_000))
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| first_house_past_fast_max_presents(36_000_000))
    }
}
