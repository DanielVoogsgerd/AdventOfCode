const RADIX: u64 = 26;
const START_CHAR: char = 'a';

const ILLEGAL_CHARS: [char; 3] = ['o', 'i', 'l'];

fn main() {
    let current_pass = "hepxcrrq";

    let next_password = next_pass(current_pass);
    println!("{:?}", next_password);

    let next_password = next_pass(&next_password);
    println!("{:?}", next_password);
}

fn next_pass(current_pass: &str) -> String {
    let mut next_attempt_number = pass_to_number(current_pass) + 1;

    let next_pass = loop {
	let next_attempt = number_to_pass(next_attempt_number);

	if is_valid_pass(&next_attempt) {
	    break next_attempt;
	}

	next_attempt_number += 1;
    };

    next_pass.into_iter().collect::<String>()

}

fn is_valid_pass(pass: &[char]) -> bool {
    if pass.iter().any(|x| ILLEGAL_CHARS.contains(x) ) {
	return false
    }

    if ! pass.windows(3).any(|window| {
	window[0] as u8 + 1 == window[1] as u8 && window[0] as u8 + 2 == window[2] as u8
    }) {
	return false;
    }

    let doubles = pass.windows(2).filter(|window| window[0] == window[1]).count();
    let triplets = pass.windows(3).filter(|window| window[0] == window[1] && window[0] == window[2]).count();

    if doubles - triplets <= 1 {
	return false;
    }

    true
}

fn number_to_pass(number: u64) -> Vec<char> {
    if number == 0 {
	return Vec::new();
    }
	
    let mut acc = number;
    let radix = RADIX;
    let mut pass: Vec<char> = Vec::new();
    
    loop {
	let digit: u8 = ((acc - 1) % (radix)) as u8;
	pass.push((digit + START_CHAR as u8) as char);
	acc = (acc - 1) / radix;

	if acc == 0 {
	    break;
	}
    }

    pass.into_iter().rev().collect::<Vec<_>>()
}    

fn pass_to_number(pass: &str) -> u64 {
    pass.chars().map(|x| (x as u64) - START_CHAR as u64 + 1).reduce(|acc, elem| {
	(acc * (RADIX)) + elem
    }).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::{number_to_pass, pass_to_number};

    #[test]
    fn number_to_pass_empty() {
	assert_eq!(number_to_pass(0), &[])
    }

    #[test]
    fn number_to_pass_a() {
	assert_eq!(number_to_pass(1), &['a'])
    }

    #[test]
    fn number_to_pass_z() {
	assert_eq!(number_to_pass(26), &['z'])
    }

    #[test]
    fn number_to_pass_aa() {
	assert_eq!(number_to_pass(27), &['a', 'a'])
    }

    #[test]
    fn number_to_pass_az() {
	assert_eq!(number_to_pass(52), &['a', 'z'])
    }

    #[test]
    fn pass_to_number_empty() {
	assert_eq!(pass_to_number(""), 0);
    }

    #[test]
    fn pass_to_number_a() {
	assert_eq!(pass_to_number("a"), 1);
    }

    #[test]
    fn pass_to_number_z() {
	assert_eq!(pass_to_number("z"), 26);
    }

    #[test]
    fn pass_to_number_aa() {
	assert_eq!(pass_to_number("aa"), 27);
    }

    // #[test]
    // fn number_to_pass_az() {
    // 	assert_eq!(number_to_pass(27), &['a', 'a'])
    // }
}
