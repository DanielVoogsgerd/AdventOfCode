use std::fs::read_to_string;

fn main() {
    part_one();
    part_two();
}


fn part_one() {
    let data = read_to_string("data/02.txt").expect("Could not read data file");

    let res = data.lines().filter_map(|line| {
	let mut tokens = line.split('x');
	let l = tokens.next()?.parse::<i64>().expect("Invalid number");
	let w = tokens.next()?.parse::<i64>().expect("Invalid number");
	let h = tokens.next()?.parse::<i64>().expect("Invalid number");

	let s1 = l*w;
	let s2 = w*h;
	let s3 = h*l;

	let smallest = i64::min(s1, i64::min(s2, s3));

	Some(2i64 * (s1 + s2 + s3) + smallest)
    }).sum::<i64>();

    println!("{}", res);
}


fn part_two() {
    let data = read_to_string("data/02.txt").expect("Could not read data file");

    let res = data.lines().filter_map(|line| {
	let mut tokens = line.split('x');
	let l = tokens.next()?.parse::<i64>().expect("Invalid number");
	let w = tokens.next()?.parse::<i64>().expect("Invalid number");
	let h = tokens.next()?.parse::<i64>().expect("Invalid number");

	let biggest = i64::max(l, i64::max(w, h));

	Some(2i64 * (l + w + h - biggest) + l * w * h)
    }).sum::<i64>();

    println!("{}", res);
}
