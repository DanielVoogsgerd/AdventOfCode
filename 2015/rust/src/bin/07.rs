use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{BufRead, BufReader},
};

type Var = String;
type Num = u16;

#[derive(PartialEq)]
enum Token {
    Value(Value),
    Instruction(InstructionType),
}

#[derive(PartialEq, Debug)]
enum Value {
    Var(Var),
    Num(Num),
}

#[derive(PartialEq)]
enum InstructionType {
    And,
    Or,
    Not,
    Rshift,
    Lshift,
}

#[derive(Debug)]
enum Instruction {
    And(Value, Value),
    Or(Value, Value),
    Not(Value),
    Rshift(Value, Num),
    Lshift(Value, Num),
    Set(Value),
}

fn main() {
    let file = File::open("./data/07.txt").expect("Could not open datafile");
    let instructions = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.expect("Could not read line");

            let mut parts = line.split(" -> ");

            let operation = parts.next().expect("Could not find LHS");
            let destination = parts.next().expect("Could not find RHS").to_owned();

            let segments = operation.split_whitespace();

            let mut tokens = segments.map(|word| match word {
                "NOT" => Token::Instruction(InstructionType::Not),
                "AND" => Token::Instruction(InstructionType::And),
                "OR" => Token::Instruction(InstructionType::Or),
                "RSHIFT" => Token::Instruction(InstructionType::Rshift),
                "LSHIFT" => Token::Instruction(InstructionType::Lshift),
                _ => match word.parse::<Num>() {
                    Ok(num) => Token::Value(Value::Num(num)),
                    Err(_) => Token::Value(Value::Var(word.to_owned())),
                },
            });

            let first_token = tokens.next().expect("Could not find first token");

            let lhs = match first_token {
                Token::Value(first_value) => {
                    if let Some(second_token) = tokens.next() {
                        if let Token::Instruction(instruction_type) = second_token {
                            match instruction_type {
                                InstructionType::And | InstructionType::Or => {
                                    if let Token::Value(second_val) = tokens.next().expect(
                                        "Could not find third token for AND/OR instruction.",
                                    ) {
                                        match instruction_type {
                                            InstructionType::And => Instruction::And(first_value, second_val),
                                            InstructionType::Or => Instruction::Or(first_value, second_val),
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        panic!("Token after AND/OR instruction should be a VAR");
                                    }
                                }
                                InstructionType::Not => {
                                    panic!("NOT instructions should always START with NOT")
                                }
                                InstructionType::Rshift | InstructionType::Lshift => {
                                    if let Token::Value(Value::Num(shift_num)) =
                                        tokens.next().expect(
                                            "Could not find third token for AND/OR instruction.",
                                        )
                                    {
                                        match instruction_type {
                                            InstructionType::Rshift => Instruction::Rshift(first_value, shift_num),
                                            InstructionType::Lshift => Instruction::Lshift(first_value, shift_num),
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        panic!("Token after RSHIFT/LSHIFT instruction should be a NUM");
                                    }
                                }
                            }
                        } else {
                            panic!("VAR tokens should always be succeeded by an INSTRUCTION TOKEN");
                        }
                    } else {
                        Instruction::Set(first_value)
                    }
                }
                Token::Instruction(InstructionType::Not) => {
                    if let Token::Value(var) = tokens
                        .next()
                        .expect("Could not find second token for NOT instruction")
                    {
                        Instruction::Not(var)
                    } else {
                        panic!("NOT Instructions have be be succeeded by a VAR");
                    }
                }
                _ => panic!("Invalid instruction"),
            };

            (lhs, destination)
        })
        .collect::<Vec<(Instruction, Var)>>();

    // Part one
    let instruction_list: LinkedList<(&Instruction, &Var)> = instructions
        .iter()
        .map(|(x, y)| (x, y))
        .collect::<LinkedList<_>>();
    let vars = run_instruction_set(instruction_list);
    let a = vars.get("a").expect("Could not get var a");

    println!("Answer part one: {}", a);

    // Part two
    let mut instruction_list = instructions
        .iter()
        .filter_map(|(x, y)| if y == "b" { None } else { Some((x, y)) })
        .collect::<LinkedList<_>>();
    let x = (Instruction::Set(Value::Num(*a)), &String::from("b"));
    instruction_list.push_front((&x.0, x.1));

    let vars = run_instruction_set(instruction_list);

    println!(
        "Answer part two: {}",
        vars.get("a").expect("Could not get var a")
    );
}

fn run_instruction_set<'a>(
    mut instruction_set: LinkedList<(&Instruction, &'a String)>,
) -> HashMap<&'a str, u16> {
    let mut vars: HashMap<&str, Num> = HashMap::new();
    loop {
        if let Some((instruction, dest)) = instruction_set.pop_front() {
            match instruction {
                Instruction::And(s_var1, s_var2) | Instruction::Or(s_var1, s_var2) => {
                    let opt_num1 = match s_var1 {
                        Value::Var(var) => vars.get(var as &str),
                        Value::Num(num) => Some(num),
                    };
                    let opt_num2 = match s_var2 {
                        Value::Var(var) => vars.get(var as &str),
                        Value::Num(num) => Some(num),
                    };
                    if let (Some(num1), Some(num2)) = (opt_num1, opt_num2) {
                        let res = match instruction {
                            Instruction::And(_, _) => num1 & num2,
                            Instruction::Or(_, _) => num1 | num2,
                            _ => unreachable!(),
                        };
                        vars.insert(dest, res);
                    } else {
                        instruction_set.push_back((instruction, dest))
                    }
                }
                Instruction::Not(s_var)
                | Instruction::Rshift(s_var, _)
                | Instruction::Lshift(s_var, _)
                | Instruction::Set(s_var) => {
                    let opt_num = match s_var {
                        Value::Var(var) => vars.get(var as &str),
                        Value::Num(num) => Some(num),
                    };

                    if let Some(&num) = opt_num {
                        let res = match instruction {
                            Instruction::Not(_) => !num,
                            Instruction::Rshift(_, shift_len) => num >> shift_len,
                            Instruction::Lshift(_, shift_len) => num << shift_len,
                            Instruction::Set(_) => num,
                            _ => unreachable!(),
                        };
                        vars.insert(dest, res);
                    } else {
                        instruction_set.push_back((instruction, dest))
                    }
                }
            }
        } else {
            eprintln!("No more instructions");
            break;
        }
    }

    vars
}
