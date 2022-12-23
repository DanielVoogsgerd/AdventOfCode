use std::{collections::HashMap, fs::read_to_string};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::{complete, is_alphabetic},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

type Value = i64;

fn main() {
    let data = read_to_string("./data/21.txt").expect("Could not load data file");
    println!("Part one: {}", part_one(&data));
    println!("Part two: {}", part_two(&data));
}

fn part_one(data: &str) -> Value {
    let operations = data
        .lines()
        .map(|line| parse_line(line).expect("Could not parse line").1);

    let mut known_data: HashMap<&str, Value> = HashMap::new();
    let mut deps: HashMap<&str, Vec<(&str, Operation)>> = HashMap::new();
    let mut todo = vec![];

    for op in operations {
        match op.1 {
            Operation::Assign(val) => {
                known_data.insert(op.0, val);
                todo.push(op.0);
            }
            Operation::Add(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Multiply(lhs, rhs)
            | Operation::Divide(lhs, rhs) => {
                deps.entry(lhs).or_default().push(op.clone());
                deps.entry(rhs).or_default().push(op);
            }
            _ => unreachable!(),
        };
    }

    while let Some(next) = todo.pop() {
        if let Some(ops) = deps.remove(next) {
            for (dest, op) in ops {
                match op {
                    Operation::Add(_, _)
                    | Operation::Subtract(_, _)
                    | Operation::Multiply(_, _)
                    | Operation::Divide(_, _) => {
                        if let Some(val) = op.calc(&known_data) {
                            known_data.insert(dest, val);
                            todo.push(dest);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    *known_data.get("root").expect("Could not find root")
}

fn part_two(data: &str) -> Value {
    let (root, operations): (Vec<(&str, Operation)>, Vec<(&str, Operation)>) = data
        .lines()
        .map(|line| parse_line(line).expect("Could not parse line").1)
        .partition(|k| k.0 == "root");

    let mut known_data: HashMap<&str, Value> = HashMap::new();
    let mut deps: HashMap<&str, Vec<(&str, Operation)>> = HashMap::new();
    let mut todo = vec![];

    for op in operations {
        match op.1 {
            Operation::Assign(val) => {
                if op.0 == "humn" {
                    continue;
                }
                known_data.insert(op.0, val);
                todo.push(op.0);
            }
            Operation::Add(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Multiply(lhs, rhs)
            | Operation::Divide(lhs, rhs) => {
                deps.entry(lhs).or_default().push(op.clone());
                deps.entry(rhs).or_default().push(op);
            }
        };
    }

    while let Some(next) = todo.pop() {
        if let Some(ops) = deps.remove(next) {
            for (dest, op) in ops {
                match op {
                    Operation::Add(_, _)
                    | Operation::Subtract(_, _)
                    | Operation::Multiply(_, _)
                    | Operation::Divide(_, _) => {
                        if let Some(val) = op.calc(&known_data) {
                            known_data.insert(dest, val);
                            todo.push(dest);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    dbg!(known_data.len());

    match root[0].1 {
        Operation::Add(lhs, rhs)
        | Operation::Subtract(lhs, rhs)
        | Operation::Multiply(lhs, rhs)
        | Operation::Divide(lhs, rhs) => match (known_data.get(lhs), known_data.get(rhs)) {
            (None, None) => panic!("Insufficient data to solve equation"),
            (None, Some(v)) => {
                todo.push(lhs);
                known_data.insert(lhs, *v);
            }
            (Some(v), None) => {
                todo.push(rhs);
                known_data.insert(rhs, *v);
            }
            (Some(_), Some(_)) => panic!("Equation already solved"),
        },
        _ => panic!("Wrong instruction of root found"),
    };

    // let mut ideps: HashMap<&str, Vec<(&str, Operation)>> = HashMap::new();

    for (dest, op) in deps.clone().into_values().flatten() {
        let new_ops = op.invert(dest);

        for op2 in new_ops {
            match op2.1 {
                Operation::Add(lhs, rhs)
                | Operation::Subtract(lhs, rhs)
                | Operation::Multiply(lhs, rhs)
                | Operation::Divide(lhs, rhs) => {
                    deps.entry(lhs).or_default().push(op2.clone());
                    deps.entry(rhs).or_default().push(op2);
                }
                _ => {}
            };
        }
    }

    while let Some(next) = todo.pop() {
        if let Some(ops) = deps.remove(next) {
            for (dest, op) in ops {
                match op {
                    Operation::Add(_, _)
                    | Operation::Subtract(_, _)
                    | Operation::Multiply(_, _)
                    | Operation::Divide(_, _) => {
                        if let Some(val) = op.calc(&known_data) {
			    if dest == "humn" {
				return val;
			    }
                            known_data.insert(dest, val);
                            todo.push(dest);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    dbg!(known_data.len());

    panic!("Could not find a valid value for humn");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation<'a> {
    Assign(Value),
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

impl<'a> Operation<'a> {
    fn calc(&self, known_data: &HashMap<&str, Value>) -> Option<Value> {
        Some(match self {
            Operation::Assign(_) => panic!("calc is not implemented for Operation"),
            Operation::Add(lhs, rhs) => known_data.get(lhs)? + known_data.get(rhs)?,
            Operation::Subtract(lhs, rhs) => known_data.get(lhs)? - known_data.get(rhs)?,
            Operation::Multiply(lhs, rhs) => known_data.get(lhs)? * known_data.get(rhs)?,
            Operation::Divide(lhs, rhs) => known_data.get(lhs)? / known_data.get(rhs)?,
        })
    }

    fn invert(self, dest: &'a str) -> [(&str, Self); 2] {
        match self {
            Operation::Add(lhs, rhs) => [
                (rhs, Operation::Subtract(dest, lhs)),
                (lhs, Operation::Subtract(dest, rhs)),
            ],
            Operation::Subtract(lhs, rhs) => [
                (rhs, Operation::Subtract(lhs, dest)),
                (lhs, Operation::Add(dest, rhs)),
            ],
            Operation::Multiply(lhs, rhs) => [
                (rhs, Operation::Divide(dest, lhs)),
                (lhs, Operation::Divide(dest, rhs)),
            ],
            Operation::Divide(lhs, rhs) => [
                (rhs, Operation::Divide(lhs, dest)),
                (lhs, Operation::Multiply(dest, rhs)),
            ],
            _ => panic!("Cannot convert this type of operation"),
        }
    }
}

fn parse_line<'a>(data: &'a str) -> IResult<&str, (&str, Operation<'a>)> {
    separated_pair(take(4usize), tag(": "), parse_operation)(data)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((parse_operator, parse_assignment))(input)
}

fn parse_operator(data: &str) -> IResult<&str, Operation> {
    let (res, tup) = tuple((
        take_while(|x| is_alphabetic(x as u8)),
        delimited(
            complete::char(' '),
            alt((
                complete::char('+'),
                complete::char('-'),
                complete::char('/'),
                complete::char('*'),
            )),
            complete::char(' '),
        ),
        take_while(|x| is_alphabetic(x as u8)),
    ))(data)?;

    let out = match tup.1 {
        '+' => Operation::Add(tup.0, tup.2),
        '-' => Operation::Subtract(tup.0, tup.2),
        '*' => Operation::Multiply(tup.0, tup.2),
        '/' => Operation::Divide(tup.0, tup.2),
        _ => unreachable!(),
    };

    IResult::Ok((res, out))
}

fn parse_assignment(data: &str) -> IResult<&str, Operation> {
    let (res, number) = complete::i64(data)?;
    IResult::Ok((res, Operation::Assign(number)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignment() {
        let (_res, (var, operation)) = parse_line("root: 5").unwrap();
        assert_eq!(var, "root");
        assert_eq!(operation, Operation::Assign(5));
    }

    #[test]
    fn test_parse_add() {
        let (_res, (var, operation)) = parse_line("root: abcd + efgh").unwrap();
        assert_eq!(var, "root");
        assert_eq!(operation, Operation::Add("abcd", "efgh"));
    }

    #[test]
    fn test_part_one() {
        let data = read_to_string("./data/21-example.txt").expect("Could not read example data");
        let answer = part_one(&data);

        assert_eq!(answer, 152);
    }

    #[test]
    fn test_part_two() {
        let data = read_to_string("./data/21-example.txt").expect("Could not read example data");
        let answer = part_two(&data);

        assert_eq!(answer, 301);
    }
}
