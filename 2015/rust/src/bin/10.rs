fn main() {
    let input = String::from("1113222113");

    let mut current = input;

    for _i in 1..=40 {
	current = look_say(&current);
    }
    println!("Answer part one: {}", current.len());

    for _i in 41..=50 {
	current = look_say(&current);
    }

    println!("Answer part two: {}", current.len());
}

fn look_say(inp: &str) -> String {
    let mut counter = 1;
    let mut output = String::new();
    let mut chars = inp.chars();

    let mut last_chr = chars.next().expect("Received an empty string");

    chars.for_each(|chr| {
	if chr == last_chr {
	    counter += 1;
	} else {
	    output.push_str(&counter.to_string());
	    output.push(last_chr);
	    counter = 1;
	    last_chr = chr;
	}
    });
    output.push_str(&counter.to_string());
    output.push(last_chr);

    output
}


#[cfg(test)]
mod tests {
    use crate::look_say;

    #[test]
    pub fn test_1() {
	assert_eq!(look_say("1"), String::from("11"));
    }

    #[test]
    pub fn test_11() {
	assert_eq!(look_say("11"), String::from("21"));
    }

    #[test]
    pub fn test_21() {
	assert_eq!(look_say("21"), String::from("1211"));
    }

    #[test]
    pub fn test_1211() {
	assert_eq!(look_say("1211"), String::from("111221"));
    }

    #[test]
    pub fn test_111221() {
	assert_eq!(look_say("111221"), String::from("312211"));
    }
}
