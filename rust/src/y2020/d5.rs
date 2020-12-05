use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        assert_eq!(parse_pass("BFFFBBFRRR"), 567);
        assert_eq!(parse_pass("FFFBBBFRRR"), 119);
        assert_eq!(parse_pass("BBFFBBFRLL"), 820);
    }
}

fn parse_pass(pass: &str) -> i32 {
    let mut bit = 0x200;
    let mut pass_id = 0;

    for c in pass.chars() {
        if c == 'B' || c == 'R' {
            pass_id |= bit;
        }
        bit >>= 1;
    }
    return pass_id;
}

fn row(pass_id: i32) -> i32 {
    return pass_id >> 3;
}

fn front_or_back(pass_id: i32) -> bool {
    let row = row(pass_id);
    return row == 127 || row == 0;
}

fn solve_2(input: &str) -> i32 {
    let filtered_pass_ids: BTreeSet<i32> = BTreeSet::from_iter(
        input
            .split("\n")
            .map(|pass| parse_pass(pass))
            .filter(|&pass_id| !front_or_back(pass_id)),
    );
    for pass_id in &filtered_pass_ids {
        if !filtered_pass_ids.get(&(pass_id + 1)).is_some()
            && filtered_pass_ids.get(&(pass_id + 2)).is_some()
        {
            return pass_id + 1;
        }
    }
    panic!();
}

fn solve_1(input: &str) -> i32 {
    return input
        .split("\n")
        .map(|pass| parse_pass(pass))
        .max()
        .unwrap();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
