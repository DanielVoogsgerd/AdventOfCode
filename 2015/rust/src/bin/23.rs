use std::{collections::HashMap, fs::read_to_string};

type Register = char;
type Offset = isize;

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let segments = value
            .split_once(' ')
            .expect("Could not find segments for instruction");
        match segments.0 {
            "hlf" | "tpl" | "inc" => {
                let register = segments
                    .1
                    .parse::<char>()
                    .map_err(|_| "Could not parse segment as register")?;
                match segments.0 {
                    "hlf" => Ok(Instruction::Half(register)),
                    "tpl" => Ok(Instruction::Triple(register)),
                    "inc" => Ok(Instruction::Increment(register)),
                    _ => unreachable!(),
                }
            }
            "jmp" => {
                let offset = segments
                    .1
                    .parse::<isize>()
                    .map_err(|_| "Could not parse segment as offset")?;
                Ok(Instruction::Jump(offset))
            }
            "jie" | "jio" => {
                let parts = segments
                    .1
                    .split_once(", ")
                    .expect("Could not find parameters for jie instruction");
                let register = parts
                    .0
                    .parse::<char>()
                    .map_err(|_| "Could not parse parameter as register")?;
                let offset = parts
                    .1
                    .parse::<isize>()
                    .map_err(|_| "Could not parse parameter as offset")?;
                match segments.0 {
                    "jie" => Ok(Instruction::JumpIfEven(register, offset)),
                    "jio" => Ok(Instruction::JumpIfOne(register, offset)),
                    _ => unreachable!(),
                }
            }
            _ => Err("Unexpected instruction"),
        }
    }
}

struct Runtime {
    cursor: isize,
    registers: HashMap<char, usize>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            cursor: 0,
            registers: HashMap::new(),
        }
    }

    fn set_register(&mut self, register: char, value: usize) {
        self.registers.insert(register, value);
    }
}

impl Runtime {
    fn run_instructions(&mut self, instructions: &[Instruction]) {
        loop {
            if self.cursor < 0 || (self.cursor as usize) >= instructions.len() {
                return;
            }
            match instructions[self.cursor as usize] {
                Instruction::Half(register) => {
                    *self.registers.entry(register).or_insert(0) /= 2;
                    self.cursor += 1;
                }
                Instruction::Triple(register) => {
                    *self.registers.entry(register).or_insert(0) *= 3;
                    self.cursor += 1;
                }
                Instruction::Increment(register) => {
                    *self.registers.entry(register).or_insert(0) += 1;
                    self.cursor += 1;
                }
                Instruction::Jump(offset) => {
                    self.cursor += offset;
                }
                Instruction::JumpIfEven(register, offset) => {
                    if *self.registers.entry(register).or_insert(0) % 2 == 0 {
                        self.cursor += offset;
                    } else {
                        self.cursor += 1;
                    }
                }
                Instruction::JumpIfOne(register, offset) => {
                    if *self.registers.entry(register).or_insert(0) == 1 {
                        self.cursor += offset;
                    } else {
                        self.cursor += 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let data = read_to_string("data/23.txt").expect("Could not read datafile");

    let instructions: Vec<Instruction> = data
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect::<Vec<_>>();

    let mut runtime = Runtime::new();
    runtime.run_instructions(&instructions);
    println!("part one: {:?}", runtime.registers.get(&'b'));

    let mut runtime = Runtime::new();
    runtime.set_register('a', 1);
    runtime.run_instructions(&instructions);
    println!("part two: {:?}", runtime.registers.get(&'b'));
}
