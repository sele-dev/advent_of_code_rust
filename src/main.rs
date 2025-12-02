use std::iter;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Instruction {
    direction: char,
    steps: i32,
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    // Parses a "line" representation of an Instruction
    // Line is assumed to be of the form: [LR][0-9]+[0-9]*\n
    // I learned that iteration over grapheme clusters is not provided by the Rust stdlib
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //println!("line: {}", line);
        let mut chars = line.trim().chars();
        let direction = chars.next().unwrap();//get first char
        let remaining: String = chars.collect();
        let steps = remaining.parse::<i32>()?;

        Ok(Instruction { direction, steps })
    }
}

fn main() {
    println!("## AoC 2025 Day 01 - Part 1");

    let filepath_part_01 = "./inputs/day_01.txt";
    let dial_size = 100; // 0 through 99
    let start_position = 50;
    let instructions: Vec<Instruction> = read_instructions(filepath_part_01).unwrap();

    // Apply each instruction and record where we end up, using scan to collect intermediate values.
    let positions = iter::once(start_position)
        .chain(instructions.into_iter().scan(start_position, |pos, instr| {
            //println!("pos before: {}", *pos);
            *pos = rotate(*pos, instr, dial_size);
            //println!("pos after: {}", *pos);
            Some(*pos)
        }))
        .collect::<Vec<i32>>();

    let num_zeroes = positions.iter().filter(|&p| *p == 0).count();

    println!("Password is: {}", num_zeroes);
}

fn read_instructions(filename: &str) -> Result<Vec<Instruction>, std::num::ParseIntError> {
    let res = read_to_string(filename)
            .unwrap()
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>, std::num::ParseIntError>>();
    return res;
}

// Using as is safe given our positive inputs?
fn rotate(current_position: i32, instr: Instruction, dial_size: i32) -> i32 {
    match instr.direction {
        'R' => ((current_position % dial_size) + (instr.steps % dial_size)) % dial_size,
        'L' => ((current_position % dial_size) - (instr.steps % dial_size) + dial_size) % dial_size,
        _ => 4, // XXX HACK
    }
}
