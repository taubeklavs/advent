use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SEQ_5: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(&parse_input(EXAMPLE_SEQ_5), 5), 127);
    }

    #[test]
    fn test_solve_2() {
        let parsed = parse_input(EXAMPLE_SEQ_5);
        let n = solve_1(&parsed, 5);
        assert_eq!(solve_2(&parsed, n), 62);
    }
}

fn is_valid_n(n: &u64, preamble: &[u64]) -> bool {
    for n1 in preamble {
        if n > n1 && preamble.contains(&(n - n1)) {
            return true;
        }
    }
    return false;
}

fn solve_1(parsed: &Vec<u64>, preamble_length: usize) -> u64 {
    let mut preamble;
    let nums = &parsed[preamble_length..];
    for (i, n) in nums.iter().enumerate() {
        preamble = &parsed[i..preamble_length + i];
        if !is_valid_n(n, &preamble) {
            return *n;
        }
    }
    panic!();
}

fn solve_2(parsed: &Vec<u64>, n: u64) -> u64 {
    for i in 0..parsed.len() {
        let mut j = i;
        let mut set_sum: u64 = 0;
        loop {
            let contiguous_set = &parsed[i..j];
            set_sum += parsed[j];
            if set_sum < n {
                j += 1;
            } else if set_sum == n {
                let min = contiguous_set.iter().min().unwrap();
                let max = contiguous_set.iter().max().unwrap();
                return min + max;
            } else {
                break;
            }
        }
    }
    panic!();
}

fn parse_input(input: &str) -> Vec<u64> {
    return input
        .split("\n")
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
}

pub fn run(input: &str) {
    let parsed = parse_input(input);
    helpers::run_benchmarked(|| println!("{}", solve_1(&parsed, 25)));
    let n = solve_1(&parsed, 25);
    helpers::run_benchmarked(|| println!("{}", solve_2(&parsed, n)));
}
