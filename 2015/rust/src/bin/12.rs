use std::{fs::read_to_string, iter::Peekable, str::Chars};

fn main() {
    part_one();
    part_one_with_tokenizer();
    part_two();
}

type Key = String;
type Pairs = (Key, Token);

#[derive(Debug, PartialEq)]
enum Token {
    Array(Vec<Token>),
    Object(Vec<Pairs>),
    String(String),
    Integer(isize),
}

#[derive(Debug, PartialEq)]
enum ParserEnvironment {
    Global,
    String,
    Integer,
    Array(ArrayEnvironment),
    Object(ObjectEnvironment),
}

#[derive(Debug, PartialEq)]
enum ArrayEnvironment {
    Before(Vec<Token>),
    After(Vec<Token>),
}

#[derive(Debug, PartialEq)]
enum ObjectEnvironment {
    Before(Vec<Pairs>),
    Intermediate(Vec<Pairs>),
    After(Vec<Pairs>, Key),
}

fn skip_whitespace(iter: &mut Peekable<Chars>) {
    while let Some(next) = iter.peek() {
        if next.is_whitespace() {
            let _ = iter.next();
        } else {
            break;
        }
    }
}

fn tokenize(input: &str) -> Option<Token> {
    let mut chr_iter = input.chars().peekable();
    let mut parser_environment = ParserEnvironment::Global;
    let mut env_stack: Vec<ParserEnvironment> = Vec::new();
    let mut prev_complete_token: Option<Token> = None;

    loop {
        skip_whitespace(&mut chr_iter);
        match parser_environment {
            ParserEnvironment::Global => {
                // This environment should be entered twice, once at the start of the parsing and once at the end.
                // We have to make sure, it does not start parsing a new object after it has finished with another one.
                if let Some(&chr) = chr_iter.peek() {
                    env_stack.push(parser_environment);
                    if let Some(env) = get_environment(chr) {
                        // Numbers are different from the other contexts.
                        // Where number is indicated by a digit or '-' sign which is part of the data
                        // the other contexts have a indicator like '{', '"', '['
                        // to indicate that start of the context.
                        if env != ParserEnvironment::Integer {
                            let _ = chr_iter.next();
                        }
                        parser_environment = env;
                    } else {
                        panic!("Illegal character between environments")
                    }
                    continue;
                } else {
                    break prev_complete_token;
                }
            }

            ParserEnvironment::String => {
                // Skipping the leading quote determined by peek
                let mut value = String::new();
                loop {
                    let opt_chr = chr_iter.next();
                    if let Some(chr) = opt_chr {
                        match chr {
                            '"' => {
                                parser_environment =
                                    env_stack.pop().expect("Could not find parent environment");
                                break;
                            }
                            _ => value.push(chr),
                        }
                    } else {
                        panic!("Ran out of data in string environment")
                    }
                }

                prev_complete_token = Some(Token::String(value));
            }

            ParserEnvironment::Integer => {
                let mut value = String::new();

                value.push(
                    chr_iter
                        .next()
                        .expect("Unreachable, was peeked by global env"),
                );

                loop {
                    let opt_chr = chr_iter.peek();
                    if let Some(&chr) = opt_chr {
                        if chr.is_ascii_digit() {
                            value.push(chr);
                            let _ = chr_iter.next();
                        } else {
                            parser_environment =
                                env_stack.pop().expect("Could not find parent environment");
                            break;
                        }
                    } else {
                        parser_environment =
                            env_stack.pop().expect("Could not find parent environment");
                        break;
                    }
                }
                prev_complete_token = Some(Token::Integer(
                    value.parse().expect("I created an invalid number"),
                ));
            }

            ParserEnvironment::Array(ArrayEnvironment::Before(current_array)) => {
                let opt_chr = chr_iter.peek();
                if let Some(&chr) = opt_chr {
                    if chr == ']' {
                        let _ = chr_iter.next();
                        parser_environment = env_stack.pop().expect("Ran out of env stack");
                        prev_complete_token = Some(Token::Array(current_array));
                    } else {
                        let new_environment = get_environment(chr);

                        if let Some(env) = new_environment {
                            if env != ParserEnvironment::Integer {
                                let _ = chr_iter.next();
                            }
                            env_stack.push(ParserEnvironment::Array(ArrayEnvironment::After(
                                current_array,
                            )));
                            match env {
                                ParserEnvironment::Integer => {
                                    parser_environment = ParserEnvironment::Integer
                                }
                                ParserEnvironment::String => {
                                    parser_environment = ParserEnvironment::String
                                }
                                ParserEnvironment::Array(new_env) => {
                                    parser_environment = ParserEnvironment::Array(new_env)
                                }
                                ParserEnvironment::Object(new_env) => {
                                    parser_environment = ParserEnvironment::Object(new_env)
                                }
                                _ => todo!(),
                            }
                        } else {
                            panic!("Could not find valid context for char {chr}")
                        }
                    }
                } else {
                    panic!("Ran out of data in string environment")
                }
            }

            ParserEnvironment::Array(ArrayEnvironment::After(mut current_array)) => {
                if let Some(prev_token) = prev_complete_token {
                    current_array.push(prev_token);
                } else {
                    panic!("No previous value found after array element");
                }
                prev_complete_token = None;

                let chr = chr_iter
                    .next()
                    .expect("Ran out of data in open array environment");
                match chr {
                    ',' => {
                        parser_environment =
                            ParserEnvironment::Array(ArrayEnvironment::Before(current_array));
                    }
                    ']' => {
                        parser_environment = env_stack.pop().expect("Ran out of env stack");
                        prev_complete_token = Some(Token::Array(current_array));
                    }
                    _ => panic!("Unexpected char: {chr} between array elements"),
                }
            }
            ParserEnvironment::Object(ObjectEnvironment::Before(current_pairs)) => {
                if let Some(chr) = chr_iter.next() {
                    if chr == '}' {
                        parser_environment = env_stack.pop().expect("Ran out of env stack");
                        prev_complete_token = Some(Token::Object(current_pairs));
                    } else if let Some(env) = get_environment(chr) {
                        if env == ParserEnvironment::String {
                            env_stack.push(ParserEnvironment::Object(
                                ObjectEnvironment::Intermediate(current_pairs),
                            ));
                            parser_environment = ParserEnvironment::String;
                        } else {
                            panic!("Object keys can only be strings");
                        }
                    } else {
                        panic!("Unexpected char {chr} in Object Environment (before key)");
                    }
                } else {
                    panic!("Ran out of data in object context");
                }
            }
            ParserEnvironment::Object(ObjectEnvironment::Intermediate(current_pairs)) => {
                let key = if let Some(Token::String(key)) = prev_complete_token {
                    if let Some(chr) = chr_iter.next() {
                        if chr != ':' {
                            panic!("Illegal char {chr} in open object (after key)")
                        }
                    }
                    skip_whitespace(&mut chr_iter);
                    key
                } else {
                    panic!("No previous value found after object key")
                };

                prev_complete_token = None;

                if let Some(&chr) = chr_iter.peek() {
                    let new_environment = get_environment(chr);

                    if let Some(env) = new_environment {
                        if env != ParserEnvironment::Integer {
                            let _ = chr_iter.next();
                        }
                        env_stack.push(ParserEnvironment::Object(ObjectEnvironment::After(
                            current_pairs,
                            key,
                        )));
                        match env {
                            ParserEnvironment::Integer => {
                                parser_environment = ParserEnvironment::Integer;
                            }
                            ParserEnvironment::String => {
                                parser_environment = ParserEnvironment::String;
                            }
                            ParserEnvironment::Array(new_env) => {
                                parser_environment = ParserEnvironment::Array(new_env);
                            }
                            ParserEnvironment::Object(new_env) => {
                                parser_environment = ParserEnvironment::Object(new_env);
                            }
                            ParserEnvironment::Global => unreachable!(),
                        }
                    } else {
                        panic!("Could not find context for char: {chr}");
                    }
                } else {
                    panic!("Ran out of data in object context");
                }
            }
            ParserEnvironment::Object(ObjectEnvironment::After(mut current_pairs, key)) => {
                if let Some(prev_token) = prev_complete_token {
                    current_pairs.push((key, prev_token));
                } else {
                    panic!("No previous value found after object value")
                }
                prev_complete_token = None;

                let chr = chr_iter
                    .next()
                    .expect("Ran out of data in open array environment");
                match chr {
                    ',' => {
                        parser_environment =
                            ParserEnvironment::Object(ObjectEnvironment::Before(current_pairs));
                    }
                    '}' => {
                        parser_environment = env_stack.pop().expect("Ran out of env stack");
                        prev_complete_token = Some(Token::Object(current_pairs));
                    }
                    _ => panic!("Unexpected char: {chr} between array elements"),
                }
            }
        };
    }
}

fn get_environment(chr: char) -> Option<ParserEnvironment> {
    match chr {
        '"' => Some(ParserEnvironment::String),
        _ if chr.is_ascii_digit() => Some(ParserEnvironment::Integer),
        '-' => Some(ParserEnvironment::Integer),
        '[' => Some(ParserEnvironment::Array(ArrayEnvironment::Before(
            Vec::new()
        ))),
        '{' => Some(ParserEnvironment::Object(ObjectEnvironment::Before(
            Vec::new(),
        ))),
        _ => None,
    }
}

fn part_one() {
    let data = read_to_string("./data/12.txt").expect("Could not read datafile");

    let segments = data.split(&[',', ':', '(', ')', '[', ']', '"', '{', '}'][..]);

    let numbers: i32 = segments
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .sum();

    println!("Part one: {numbers}");
}

fn part_one_with_tokenizer() {
    let data = read_to_string("./data/12.txt").expect("Could not read datafile");
    let tokens = tokenize(&data).expect("expected tokens");

    let mut token_stack: Vec<&Token> = vec![&tokens];

    let mut running_sum = 0;

    while let Some(token) = token_stack.pop() {
        match token {
            Token::String(_) => {}
            Token::Integer(val) => running_sum += val,
            Token::Array(elements) => {
                for element in elements {
                    token_stack.push(element);
                }
            }
            Token::Object(elements) => {
                for (_key, value) in elements {
                    token_stack.push(value);
                }
            }
        }
    }

    println!("Part one (with tokenizer): {running_sum}");
}

fn part_two() {
    let data = read_to_string("./data/12.txt").expect("Could not read datafile");
    let tokens = tokenize(&data).expect("expected tokens");

    let mut token_stack: Vec<&Token> = vec![&tokens];

    let mut running_sum = 0;

    while let Some(token) = token_stack.pop() {
        match token {
            Token::Integer(val) => running_sum += val,
            Token::Array(elements) => {
                for element in elements {
                    token_stack.push(element);
                }
            }
            Token::Object(elements) => {
                let mut found_red = false;
                for (_key, value) in elements {
                    if let Token::String(str_val) = value {
                        if str_val == "red" {
                            found_red = true;
                            break;
                        }
                    }
                }
                if found_red {
                    continue;
                }
                for (_key, value) in elements {
                    token_stack.push(value);
                }
            }
            _ => {}
        }
    }

    println!("Part two (with tokenizer): {running_sum}");
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, Token};

    #[test]
    fn tokenize_string() {
        assert_eq!(tokenize("\"hi\""), Some(Token::String("hi".into())))
    }

    #[test]
    fn tokenize_integer() {
        assert_eq!(tokenize("123"), Some(Token::Integer(123)))
    }

    #[test]
    fn tokenize_negative_integer() {
        assert_eq!(tokenize("-123"), Some(Token::Integer(-123)))
    }

    #[test]
    fn tokenize_empty_array() {
        assert_eq!(tokenize("[]"), Some(Token::Array(Vec::new())))
    }

    #[test]
    fn tokenize_array_numeric_single() {
        assert_eq!(tokenize("[1]"), Some(Token::Array(vec![Token::Integer(1)])))
    }

    #[test]
    fn tokenize_array_numeric_multiple() {
        assert_eq!(
            tokenize("[1,2]"),
            Some(Token::Array(vec![Token::Integer(1), Token::Integer(2)]))
        )
    }

    #[test]
    fn tokenize_array_string_single() {
        assert_eq!(
            tokenize("[\"hoi\"]"),
            Some(Token::Array(vec![Token::String("hoi".to_string())]))
        )
    }

    #[test]
    fn tokenize_array_string_multiple() {
        assert_eq!(
            tokenize("[\"hoi\",\"doei\"]"),
            Some(Token::Array(vec![
                Token::String("hoi".into()),
                Token::String("doei".into())
            ]))
        )
    }

    #[test]
    fn tokenize_nested_empty_array() {
        assert_eq!(
            tokenize("[[]]"),
            Some(Token::Array(vec![Token::Array(Vec::new())]))
        );
        assert_eq!(
            tokenize("[[[]]]"),
            Some(Token::Array(vec![Token::Array(vec![Token::Array(vec![])])]))
        );
    }

    #[test]
    fn tokenize_nested_array() {
        assert_eq!(
            tokenize("[[1], [2]]"),
            Some(Token::Array(vec![
                Token::Array(vec![Token::Integer(1)]),
                Token::Array(vec![Token::Integer(2)])
            ]))
        );
        assert_eq!(
            tokenize("[[1], 2]"),
            Some(Token::Array(vec![
                Token::Array(vec![Token::Integer(1)]),
                Token::Integer(2)
            ]))
        );
    }

    #[test]
    fn tokenize_whitespace() {
        let empty_array = Some(Token::Array(Vec::new()));
        assert_eq!(tokenize(" []"), empty_array);
        assert_eq!(tokenize("[] "), empty_array);
        assert_eq!(tokenize("[ ]"), empty_array);

        let array_one_two = Some(Token::Array(vec![Token::Integer(1), Token::Integer(2)]));
        assert_eq!(tokenize("[1 ,2]"), array_one_two);
        assert_eq!(tokenize("[1, 2]"), array_one_two);
        assert_eq!(tokenize("[ 1,2]"), array_one_two);
        assert_eq!(tokenize("[1,2 ]"), array_one_two);
    }

    #[test]
    fn tokenize_empty_object() {
        assert_eq!(tokenize("{}"), Some(Token::Object(vec![])));
    }

    #[test]
    fn tokenize_object() {
        assert_eq!(
            tokenize("{\"a\": 12}"),
            Some(Token::Object(vec![("a".into(), Token::Integer(12))]))
        );
    }

    #[test]
    fn tokenize_object_in_array() {
        assert_eq!(
            tokenize("[{}]"),
            Some(Token::Array(vec![Token::Object(vec![])]))
        );
    }

    #[test]
    fn tokenize_array_in_object() {
        assert_eq!(
            tokenize("{ \"array\": [] }"),
            Some(Token::Object(vec![("array".into(), Token::Array(vec![]))]))
        );
    }
}
