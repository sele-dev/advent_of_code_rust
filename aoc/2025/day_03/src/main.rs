use anyhow::Result;
use std::fs::read_to_string;
use util::*;

#[derive(Clone, Debug)]
struct Bank {
    batteries: Vec<u32>, // TODO: handle more efficient size
}

impl TryFrom<&str> for Bank {
    type Error = anyhow::Error;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let batteries: Vec<u32> = val.to_string().chars().map(u32::from).collect();
        Ok(Bank { batteries })
    }
}

fn read_banks(filename: &str) -> Result<Vec<Bank>> {
    let res = read_to_string(filename)
        .unwrap()
        .lines()
        .map(Bank::try_from)
        .collect::<Result<Vec<Bank>>>();
    return res;
}

fn main() -> Result<()> {
    let input_filepath = get_inputs_filepath("example_day_03.txt");
    let banks: Vec<Bank> = read_banks(input_filepath.as_str())?;
    let total_max_joltage = 0;

    for bank in banks {
        for battery in bank.batteries {
            print!("{}", battery);
        }
        println!("");
    }

    println!("## AoC 2025 Day 03 - Part 1");
    println!("total maximum joltage: {}", total_max_joltage);

    //println!("## AoC 2025 Day 03 - Part 2");
    //println!("tbd!: {}", tbd);

    Ok(())
}
