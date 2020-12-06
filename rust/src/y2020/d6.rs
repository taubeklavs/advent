use std::collections::BTreeSet;
use std::collections::HashMap;

use crate::helpers;

fn unique_group_answers(group_answers: &str) -> i32 {
    let mut seen_answers: BTreeSet<char> = BTreeSet::new();
    for c in group_answers.chars() {
        if c >= 'a' && c <= 'z' {
            seen_answers.insert(c);
        }
    }
    return seen_answers.len() as i32;
}

fn yes_group_answers(group_answers: &str) -> i32 {
    let mut answer_counts: HashMap<char, i32> = HashMap::new();
    let mut required_count = 1;
    for c in group_answers.chars() {
        if c >= 'a' && c <= 'z' {
            let c_cnt = answer_counts.get_mut(&c);
            if c_cnt.is_some() {
                *c_cnt.unwrap() += 1;
            } else {
                answer_counts.insert(c, 1);
            }
        } else if c == ' ' {
            required_count += 1;
        }
    }
    return answer_counts
        .iter()
        .filter(|(_, &cnt)| cnt == required_count)
        .count() as i32;
}

fn solve_1(input: &str) -> i32 {
    return input.split("\n").map(unique_group_answers).sum();
}

fn solve_2(input: &str) -> i32 {
    return input.split("\n").map(yes_group_answers).sum();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
