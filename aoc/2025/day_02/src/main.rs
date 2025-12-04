use anyhow::Result;
use itertools::*;
use prse::parse;
use std::fs::read_to_string;
use util::get_inputs_filepath;

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl TryFrom<&str> for Range {
    type Error = anyhow::Error;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let (start, end): (i64, i64) = parse!(*val, "{}-{}");
        Ok(Range { start, end })
    }
}

fn read_ranges(filename: &str) -> Result<Vec<Range>> {
    let res = read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|piece| Range::try_from(piece))
        .collect::<Result<Vec<Range>>>();
    return res;
}

fn is_invalid_p1(id: i64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.chars().count();
    if id_len % 2 != 0 {
        // Can't have an invalid pattern if we're not of even length.
        return false;
    } else {
        let (front, back) = id_str.split_at(id_len / 2);
        return front == back;
    }
}

// Invalid IDs are only comprised of repeating groups.
fn is_invalid_p2(id: i64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.chars().count();

    let mut invalid = false;
    for group_size in (1..id_len).step_by(1) {
        let initial_group = id_str.get(0..group_size).unwrap();
        invalid = id_str
            .chars()
            .chunks(group_size)
            .into_iter()
            .all(|group| group.collect::<String>() == initial_group);
        if invalid {
            break;
        };
    }
    return invalid;
}

fn main() -> Result<()> {
    let input_filepath = get_inputs_filepath("day_02.txt");
    let ranges: Vec<Range> = read_ranges(input_filepath.as_str())?;

    // Find and add up invalid IDs
    let invalids_total_p1 = ranges.iter().fold(0, |final_total, range| {
        let sum = (range.start..=range.end)
            .into_iter()
            .fold(0, |range_total, elem| {
                if is_invalid_p1(elem) {
                    range_total + elem
                } else {
                    range_total + 0
                }
            });
        final_total + sum
    });

    let invalids_total_p2 = ranges.iter().fold(0, |final_total, range| {
        let sum = (range.start..=range.end)
            .into_iter()
            .fold(0, |range_total, elem| {
                // println!("p2 testing id: {}", elem);
                if is_invalid_p2(elem) {
                    // println!("p2 invalid id: {}", elem);
                    range_total + elem
                } else {
                    range_total + 0
                }
            });
        final_total + sum
    });

    println!("## AoC 2025 Day 02 - Part 1");
    println!("accumulated invalid IDs: {}", invalids_total_p1);

    println!("## AoC 2025 Day 02 - Part 2");
    println!("accumulated invalid IDs: {}", invalids_total_p2);

    Ok(())
}
