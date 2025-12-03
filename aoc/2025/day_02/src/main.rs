use anyhow::Result;
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


fn is_invalid(id: i64) ->  bool {
    let id_str = id.to_string();
    let id_len = id_str.len();
    if id_len % 2 != 0 {
        // Can't have an invalid pattern if we're not of even length.
        return false;
    } else {
        let (front, back) = id_str.split_at(id_len / 2);
        return front == back;
    }
}

fn main() -> Result<()> {
    let input_filepath = get_inputs_filepath("day_02.txt");
    let ranges: Vec<Range> = read_ranges(input_filepath.as_str())?;

    // Find and add up invalid IDs
    let invalids_total = ranges.iter().fold(0, |total, range| {
        let sum = (range.start..=range.end).into_iter().fold(0, |elem_total, elem| {
            if is_invalid(elem) {
                elem_total + elem
            } else {
                elem_total + 0
            }
        });
        total + sum
    });

    println!("## AoC 2025 Day 02 - Part 1");
    println!("accumulated invalid IDs: {}", invalids_total);

    Ok(())
}
