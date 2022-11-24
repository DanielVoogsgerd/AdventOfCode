fn main() {
    part_one();
    part_two();
}

fn part_one() {
    // let input = "abcdef";
    let input = "bgvyzdsv";

    let mut i = 0;

    loop {
	let hash = md5::compute(format!("{input}{i}"));

	if format!("{hash:x}").chars().take(5).all(|x| x == '0') {
	    break;
	}
	i += 1;
    }

    println!("Answer: {i}");
}

fn part_two() {
    // let input = "abcdef";
    let input = "bgvyzdsv";

    let mut i = 0;

    loop {
	let hash = md5::compute(format!("{input}{i}"));

	if format!("{hash:x}").chars().take(6).all(|x| x == '0') {
	    break;
	}
	i += 1;
    }

    println!("Answer: {i}");
}
