use std::collections::HashSet;
use std::iter::FromIterator;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum_of_2020_naive_1() {
        assert_eq!(
            find_multiplication_of_sum_of_2020_naive(&vec![1721, 979, 366, 299, 675, 1456], 2),
            514579
        );
    }

    #[test]
    fn test_find_sum_of_2020_naive_2() {
        assert_eq!(
            find_multiplication_of_sum_of_2020_naive(&vec![1721, 979, 366, 299, 675, 1456], 3),
            241861950
        );
    }

    #[test]
    fn test_find_sum_of_2020_optimized_1() {
        assert_eq!(
            find_multiplication_of_sum_of_2020_optimized(
                &HashSet::from_iter([1721, 979, 366, 299, 675, 1456].iter().cloned()),
                2,
                2020
            )
            .unwrap(),
            514579
        );
    }

    #[test]
    fn test_find_sum_of_2020_optimized_2() {
        assert_eq!(
            find_multiplication_of_sum_of_2020_optimized(
                &HashSet::from_iter([1721, 979, 366, 299, 675, 1456].iter().cloned()),
                3,
                2020
            )
            .unwrap(),
            241861950
        );
    }
}

fn find_expenses_of_sum_of_2020(
    expenses: &Vec<i32>,
    expenses_so_far: &mut Vec<i32>,
    current_index: usize,
    current_level: usize,
    max_level: usize,
) -> Option<Vec<i32>> {
    let mut i = current_index;
    let expense_count = expenses.len();
    while i < expense_count {
        expenses_so_far[current_level] = expenses[i];
        if current_level < max_level {
            find_expenses_of_sum_of_2020(
                &expenses,
                expenses_so_far,
                i + 1,
                current_level + 1,
                max_level,
            );
        }
        if expenses_so_far.iter().fold(0, |sum, x| sum + x) == 2020 {
            return Some(expenses_so_far.to_vec());
        }
        i += 1;
    }
    return None;
}

fn find_multiplication_of_sum_of_2020_naive(expenses: &Vec<i32>, count: i32) -> i32 {
    return find_expenses_of_sum_of_2020(
        expenses,
        &mut vec![0; count as usize],
        0,
        0,
        (count - 1) as usize,
    )
    .unwrap()
    .iter()
    .fold(1, |res, expense| res * expense);
}

fn find_multiplication_of_sum_of_2020_optimized(
    expense_set: &HashSet<i32>,
    count: i32,
    goal: i32,
) -> Option<i32> {
    let closure = |v: &i32, goal: &i32, cnt: &i32| -> Option<i32> {
        if *cnt > 2 {
            let vs =
                find_multiplication_of_sum_of_2020_optimized(&expense_set, count - 1, goal - v);
            if vs.is_some() {
                return Some(v * vs.unwrap());
            } else {
                return None;
            }
        } else {
            let v2 = expense_set.get(&(goal - v));
            if v2.is_some() {
                return Some(v * v2.unwrap());
            } else {
                return None;
            }
        }
    };
    return expense_set.iter().find_map(|v| closure(&v, &goal, &count));
}

fn parse_input(input: &str) -> Vec<i32> {
    return input
        .to_string()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

pub fn run(input: &str) {
    let expenses = parse_input(input);
    let p1_naive = || println!("{}", find_multiplication_of_sum_of_2020_naive(&expenses, 2));
    let p2_naive = || println!("{}", find_multiplication_of_sum_of_2020_naive(&expenses, 3));
    helpers::run_benchmarked(p1_naive);
    helpers::run_benchmarked(p2_naive);
    let p1_optimized = || {
        println!(
            "{}",
            find_multiplication_of_sum_of_2020_optimized(
                &HashSet::from_iter(expenses.iter().cloned()),
                2,
                2020
            )
            .unwrap()
        )
    };
    helpers::run_benchmarked(p1_optimized);
    let p2_optimized = || {
        println!(
            "{}",
            find_multiplication_of_sum_of_2020_optimized(
                &HashSet::from_iter(expenses.iter().cloned()),
                3,
                2020
            )
            .unwrap()
        )
    };
    helpers::run_benchmarked(p2_optimized);
}
