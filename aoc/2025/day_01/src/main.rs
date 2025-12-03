use std::fs::read_to_string;
use std::str::FromStr;
use anyhow::{ anyhow, Result };

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("Valid directions may only be from the chars 'L' or 'R'!"))
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    // Parses a "line" representation of an Instruction
    // Line is assumed to be of the form: [LR][0-9]+[0-9]*\n
    // I learned that iteration over grapheme clusters is not provided by the Rust stdlib
    fn from_str(line: &str) -> Result<Self> {
        let mut chars = line.trim().chars();
        let direction = Direction::try_from(chars.next().unwrap())?; // don't do this at home, kids
        let remaining: String = chars.collect();
        let steps = remaining.parse::<i32>()?;

        Ok(Instruction { direction, steps })
    }
}

#[derive(Clone, Copy)]
struct DialState {
    num_positions: i32,
    times_at_zero: i32,
    current_position: i32,
}

impl DialState {
    fn initialize(num_positions: i32, start_position: i32) -> DialState {
        DialState {
            num_positions: num_positions,
            times_at_zero: if start_position == 0 { 1 } else { 0 },
            current_position: start_position,
        }
    }

    fn rotate(&mut self, instr: Instruction) {
        match instr.direction {
            Direction::Left => self.rotate_left(instr.steps),
            Direction::Right => self.rotate_right(instr.steps),
        }
    }

    fn set_position(&mut self, new_position: i32) {
        self.current_position = new_position;
    }

    fn rotate_left(&mut self, steps: i32) {
        // Wish I'd found out about rem_euclid vs % a little earlier! Makes sense in hindsight - when in Rome.
        let new_position = (self.current_position - steps).rem_euclid(self.num_positions);
        // "cross" = land on (cross onto) or go past
        let times_crossed = if self.current_position > 0 {
            // Don't forget the first time we cross position zero!
            steps / self.num_positions + (if steps % self.num_positions >= self.current_position { 1 } else { 0 })
        } else {
            steps / self.num_positions
        };

        self.set_position(new_position);
        self.times_at_zero += times_crossed;
    }

    fn rotate_right(&mut self, steps: i32) {
        let new_position = (self.current_position + steps).rem_euclid(self.num_positions);
        // "cross" = land on (cross onto) or go past
        let times_crossed = (self.current_position + steps) / self.num_positions;


        self.set_position(new_position);
        self.times_at_zero += times_crossed;
    }
}

fn main() -> Result<()> {
    let num_positions = 100; // 0 through 99
    let start_position = 50;

    let mut dial_state = DialState::initialize(num_positions, start_position);

    let filepath_part_01 = "./inputs/day_01.txt";
    // let filepath_part_01 = "./inputs/initial_day_01.txt";
    let instructions: Vec<Instruction> = read_instructions(filepath_part_01)?;
    let mut positions: Vec<i32> = Vec::new();

    // Apply each instruction and record where we end up
    // First, account for the start position
    positions.push(dial_state.current_position);
    for instruction in instructions {
        dial_state.rotate(instruction);
        positions.push(dial_state.current_position);
    }

    let num_zeroes = positions.iter().filter(|&p| *p == 0).count();
    println!("## AoC 2025 Day 01 - Part 1");
    println!("Password is: {}", num_zeroes);

    println!("## AoC 2025 Day 01 - Part 2");
    println!("Password is: {}", dial_state.times_at_zero);

    Ok(())
}

fn read_instructions(filename: &str) -> Result<Vec<Instruction>> {
    let res = read_to_string(filename)
            .unwrap()
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>>>();
    return res;
}
