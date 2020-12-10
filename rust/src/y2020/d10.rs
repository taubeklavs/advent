use crate::helpers;
use std::cmp;
use std::iter;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_1), 35);
        assert_eq!(solve_1(EXAMPLE_2), 220);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_1), 8);
        assert_eq!(solve_2(EXAMPLE_2), 19208);
    }
}

fn solve_1(input: &str) -> u64 {
    let mut current_joltage = 0;
    let mut one_jolt_increases = 0;
    let mut three_jolt_increases = 1;
    let mut adapters: Vec<u64> = input
        .split("\n")
        .map(|adapter| adapter.parse::<u64>().unwrap())
        .collect();
    adapters.sort_unstable();
    for adapter in adapters {
        match adapter - current_joltage {
            1 => one_jolt_increases += 1,
            3 => three_jolt_increases += 1,
            _ => (),
        }
        current_joltage = adapter;
    }
    return one_jolt_increases * three_jolt_increases;
}

fn reachable_adapters(idx: &usize, adapters: &Vec<i64>) -> Vec<usize> {
    let mut reachable_adapters: Vec<usize> = Vec::new();
    for i in idx + 1..cmp::min(idx + 4, adapters.len()) {
        if adapters[i] - adapters[*idx] <= 3 {
            reachable_adapters.push(i);
        } else {
            break;
        }
    }
    return reachable_adapters;
}

fn find_all_chains(idx: &usize, adapters: &Vec<i64>, mut path_cnts: &mut Vec<i64>) -> i64 {
    let next_in_chain = reachable_adapters(idx, &adapters);
    if !next_in_chain.is_empty() {
        let path_cnt = next_in_chain
            .iter()
            .map(|reachable_idx| {
                if path_cnts[*reachable_idx] != -1 {
                    return path_cnts[*reachable_idx];
                } else {
                    return find_all_chains(reachable_idx, &adapters, &mut path_cnts);
                }
            })
            .sum();
        path_cnts[*idx] = path_cnt;
        return path_cnt;
    } else {
        if *idx == adapters.len() - 1 {
            return 1;
        } else {
            return 0;
        }
    }
}

fn solve_2(input: &str) -> i64 {
    let mut adapters: Vec<i64> = iter::once(0)
        .chain(
            input
                .split("\n")
                .map(|adapter| adapter.parse::<i64>().unwrap()),
        )
        .collect();
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);
    let mut path_cnts = vec![-1; adapters.len()];
    return find_all_chains(&0, &adapters, &mut path_cnts);
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
