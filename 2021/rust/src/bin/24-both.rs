use std::collections::HashMap;
use std::error::Error;

fn main() {
    let file = std::fs::read_to_string("./24-input.txt").expect("Could not parse file");
    let instructions = parse_instructions(&file).unwrap();

    // println!("Instructions: {:?}", instructions);
    // run(&instructions, get_input_from_stdin).unwrap();

    let inputs: Vec<i64> = vec![9, 1, 9, 7, 2, 3, 9, 5, 9, 1, 9, 9, 9, 3];
    let inputs: Vec<i64> = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];

    if let Ok(res) = run(&instructions, &inputs) {
        if *res.get(&'z').unwrap() == 0 {
            println!("Found number");
        } else {
            println!("Nope");
        }
    } else {
        eprintln!("Something went wrong")
    }
}

fn run(
    instructions: &Vec<Expression>,
    input: &Vec<i64>,
) -> Result<HashMap<char, i64>, Box<dyn Error>> {
    let mut variables: HashMap<char, i64> = HashMap::new();
    let mut read_index: usize = 0;

    for (instruction, value1, value2) in instructions {
        // I think the parameters should be part of the instruction.
        // This would then be handled by the typesystem

        let value1: char = {
            if let Value::Variable(variable) = value1 {
                Ok(*variable)
            } else {
                // Okay this is just terrible now
                Err("First value must be a variable")
            }?
        };

        match instruction {
            Instruction::INP => {
                // let mut input: String = String::new();
                // std::io::stdin().read_line(&mut input).expect("Test");
                // let input = input.trim();
                // let value = input.parse().expect("Could not parse int");
                //
                let input_digit = input[read_index];
                read_index += 1;
                variables.insert(value1, input_digit);
            }
            Instruction::ADD => {
                let value2 = value2
                    .as_ref()
                    .ok_or("Missing second variable")?
                    .get_value(&variables);
                let value = variables.entry(value1).or_insert(0);

                *value += value2;
            }
            Instruction::MUL => {
                let value2 = value2
                    .as_ref()
                    .ok_or("Missing second variable")?
                    .get_value(&variables);
                let value = variables.entry(value1).or_insert(0);

                *value *= value2;
            }
            Instruction::DIV => {
                let value2 = value2
                    .as_ref()
                    .ok_or("Missing second variable")?
                    .get_value(&variables);
                let value = variables.entry(value1).or_insert(0);

                *value /= value2;
            }
            Instruction::MOD => {
                let value2 = value2
                    .as_ref()
                    .ok_or("Missing second variable")?
                    .get_value(&variables);
                let value = variables.entry(value1).or_insert(0);

                *value %= value2;
            }
            Instruction::EQL => {
                let value2 = value2
                    .as_ref()
                    .ok_or("Missing second variable")?
                    .get_value(&variables);
                let value = variables.entry(value1).or_insert(0);

                *value = if *value == value2 { 1 } else { 0 };
            }
        }
    }

    Ok(variables)
}

fn get_input_from_stdin() -> Result<i64, Box<dyn Error>> {
    let mut input = String::new();
    println!("Please provide input: ");
    std::io::stdin().read_line(&mut input)?;
    let value = input.trim().parse::<i64>()?;
    Ok(value)
}

type Expression = (Instruction, Value, Option<Value>);
type InstructionSet = Vec<Expression>;

fn parse_instructions(input: &str) -> Result<InstructionSet, &str> {
    let lines = input.lines();
    let mut instructions: InstructionSet = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let instruction_str = parts.next().ok_or("Line does not contain instruction")?;
        let instruction = Instruction::parse(instruction_str)?;
        let value1 = Value::parse(parts.next().ok_or("Line does not contain value")?)?;

        if let Value::Variable(_) = value1 {
        } else {
            return Err("First value must be a variable");
        }

        let value2: Option<Value> = {
            let opt_value2_str = parts.next();
            if let Some(value2_str) = opt_value2_str {
                Ok(Some(Value::parse(value2_str)?))
            } else {
                Ok(None)
            }
        }?;

        instructions.push((instruction, value1, value2))
    }

    Ok(instructions)
}

#[derive(Debug)]
enum Value {
    Variable(char),
    Value(i64),
}

impl Value {
    fn parse(value_input: &str) -> Result<Self, &str> {
        let digit = value_input.parse::<i64>();
        if let Ok(digit) = digit {
            Ok(Value::Value(digit))
        } else {
            Ok(Value::Variable(
                value_input
                    .chars()
                    .next()
                    .ok_or("No characters in variable")?,
            ))
        }
    }

    fn get_value(&self, collection: &HashMap<char, i64>) -> i64 {
        match self {
            Value::Variable(var) => {
                if let Some(val) = collection.get(&var) {
                    *val
                } else {
                    0
                }
            }
            Value::Value(x) => *x,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

impl Instruction {
    fn parse(instruction_str: &str) -> Result<Self, &str> {
        match instruction_str {
            "inp" => Ok(Instruction::INP),
            "add" => Ok(Instruction::ADD),
            "mul" => Ok(Instruction::MUL),
            "div" => Ok(Instruction::DIV),
            "mod" => Ok(Instruction::MOD),
            "eql" => Ok(Instruction::EQL),
            _ => Err("Invalid instruction"),
        }
    }
}
