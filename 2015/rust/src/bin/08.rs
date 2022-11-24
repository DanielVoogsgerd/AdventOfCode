use std::fs::read_to_string;

fn main() {
    // let file = File::open("./data/08.txt").expect("Could not open datafile");
    // let string_tuples: Vec<(String, String)> = BufReader::new(file)
        // .lines().filter_map(|line| {
    let data = read_to_string("./data/08.txt").expect("Could not open datafile");

    let answer: usize = data.lines().map(|line| {
	let mem_string = parse_string(line);
	(line, mem_string)
    }).map(|(code_string, mem_string)| code_string.chars().count() - mem_string.chars().count()).sum::<_>();

    println!("Answer part one: {answer}");


    let answer: usize = data.lines().map(|line|{
	encode_string(line).chars().count() - line.chars().count()
    }).sum();

    println!("Answer part two: {answer}");

}

pub fn parse_string(inp: &str) -> String {
    let mut chars = inp.chars();
    let mut mem_string = String::new();

    if chars.next().expect("Line is empty") != '"' {
	panic!("Strings should start with a \"");
    }

    loop {
	let next = chars.next().expect("Insufficient chars");

	match next {
	    '"' => break,
	    '\\' => {
		let escape_seq_type = chars.next().expect("Unexpected end of string");
		match escape_seq_type {
		    'x' => {
			let hex = chars.by_ref().take(2).collect::<String>();
			let binary = u32::from_str_radix(&hex, 16).expect("Could not convert hex escape to binary");
			let escape_char = char::from_u32(binary).expect("Could not convert hex escape sequence to UTF-8 char");

			mem_string.push(escape_char)
		    }
		    '\\' => {mem_string.push('\\');}
		    '"' => {mem_string.push('"');}
		    _ => {panic!("Unexpected escape sequence {escape_seq_type}");}
		}
	    }
	    _ => {
		mem_string.push(next)
	    }
	}
    }
    if let Some(last) = chars.next() {
	panic!("String ended prematurely. It was followed by char \"{last}\"");
    }

    mem_string
}

pub fn encode_string(inp: &str) -> String {
    let mut output = String::new();

    output.push('"');

    let chars = inp.chars();
    
    for next in chars {
	match next {
	    '"' => output.push_str("\\\""),
	    '\\' => output.push_str("\\\\"),
	    _ => output.push(next),
	}	
    }

    output.push('"');
    output
}


#[cfg(test)]
mod tests {
    use crate::{parse_string, encode_string};

    #[test]
    fn decode_empty() {
	assert_eq!(parse_string("\"\""), "");
    }

    #[test]
    fn decode_normal() {
	assert_eq!(parse_string("\"abc\""), "abc");
    }

    #[test]
    fn decode_escaped_quote() {
	assert_eq!(parse_string("\"aaa\\\"aaa\""), "aaa\"aaa");
    }

    #[test]
    fn decode_escaped_hex() {
	assert_eq!(parse_string("\"\x27\"").len(), 1);
	assert_eq!(parse_string("\"\x27\"") , "'");
    }

    #[test]
    fn encode_empty() {
	assert_eq!(encode_string("\"\""), "\"\\\"\\\"\"")
    }

    #[test]
    fn encode_normal() {
	assert_eq!(encode_string("\"abc\""), "\"\\\"abc\\\"\"")
    }

    #[test]
    fn encode_escaped_quote() {
	assert_eq!(encode_string("\"aaa\\\"aaa\""), "\"\\\"aaa\\\\\\\"aaa\\\"\"")
    }

    #[test]
    fn encode_escaped_hex() {
	assert_eq!(encode_string("\"\\x27\""), "\"\\\"\\\\x27\\\"\"")
    }
}
