use anyhow::Result;
use std::cmp::Ordering;
use std::fs::read_to_string;
use util::*;

#[derive(Eq, Copy, Clone, Debug)]
enum State {
    On,
    Off,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Eq, Copy, Clone, Debug)]
struct Battery {
    level: u32,
    state: State,
}

impl Battery {
    fn new(level: u32) -> Battery {
        Battery {
            level: level,
            state: State::Off,
        }
    }
}

impl Ord for Battery {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.level.cmp(&other.level))
    }
}

impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

#[derive(Clone, Debug)]
struct Bank {
    batteries: Vec<Battery>, // TODO: handle more efficient size
}

impl TryFrom<&str> for Bank {
    type Error = anyhow::Error;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        // TODO safely parse into u8's given valid input should be [0, 9] per battery level
        let batteries: Vec<Battery> = val
            .to_string()
            .chars()
            .map(|ch| Battery::new(char::to_digit(ch, 10).unwrap()))
            .collect::<Vec<Battery>>();
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

fn max_with_index<I>(iter: I) -> Option<(usize, I::Item)>
where
    I: IntoIterator + Clone,
    I::Item: Ord,
{
    let (_max_index, rmax) = match iter
        .clone()
        .into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
    {
        Some((max_index, rmax)) => (max_index, rmax),
        None => return None,
    };
    // XXX Hack - short circuit to the earliest max in the iter, since `max_by` otherwise returns the last max.
    return Some((iter.into_iter().position(|e| e == rmax).unwrap(), rmax));
}

fn top_two_with_indicies<I>(iter: I) -> Option<((usize, I::Item), (usize, I::Item))>
where
    I: IntoIterator + Clone,
    I::Item: Ord,
{
    let (first_idx, first) = match max_with_index(iter.clone()) {
        Some((first_idx, first)) => (first_idx, first),
        None => return None,
    };

    // XXX Hack - this is just tricking `max_by` instead of actually addressing this from a good approach.
    let (_second_idx, second): (usize, I::Item) =
        match iter
            .clone()
            .into_iter()
            .enumerate()
            .max_by(|(i, a), (j, b)| {
                if *i == first_idx {
                    Ordering::Less
                } else if *j == first_idx {
                    Ordering::Greater
                } else {
                    a.cmp(b)
                }
            }) {
            Some((second_idx, second)) => (second_idx, second),
            None => return None,
        };
    // XXX Hack - again we short circuit with find instead of addressing `max_by`
    Some((
        (first_idx, first),
        (iter.into_iter().position(|e| e == second).unwrap(), second),
    ))
}

fn main() -> Result<()> {
    let input_filepath = get_inputs_filepath("day_03.txt");
    let banks: Vec<Bank> = read_banks(input_filepath.as_str())?;
    let mut total_joltage = 0;

    for bank in banks {
        // XXX Indicies are never updated to match mutations. There's definitely a cleaner way of doing this.
        let ((first_idx, mut first), (second_idx, mut second)) =
            top_two_with_indicies(bank.batteries.clone()).unwrap();

        if (second_idx < first_idx) && ((first_idx + 1) != bank.batteries.len()) {
            // Select new second max battery level and index from the right-hand side

            // TODO Handle this more efficiently than cloning.
            let (_third_idx, third) =
                max_with_index(bank.batteries.clone().split_off(first_idx + 1)).unwrap();
            second = third;
        } else if (first_idx + 1) == bank.batteries.len() {
            // Just swap when the first max is at the end
            std::mem::swap(&mut first, &mut second);
        }

        let mut joltage_str = first.level.to_string();

        joltage_str.push_str(second.level.to_string().as_str());
        let joltage = str::parse::<u32>(&joltage_str)?;

        println!("max for line: {}", joltage);
        total_joltage += joltage;
    }

    println!("## AoC 2025 Day 03 - Part 1");
    println!("total maximum joltage: {}", total_joltage);

    //println!("## AoC 2025 Day 03 - Part 2");
    //println!("tbd!: {}", tbd);

    Ok(())
}
