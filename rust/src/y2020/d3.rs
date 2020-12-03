use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_INPUT), 7)
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_INPUT), 336)
    }
}

fn solve(input: &str, right: usize, down: usize) -> i64 {
    let mut lines = input.split("\n").peekable();
    let line_length = lines.peek().unwrap().chars().count();
    let res = lines
        .enumerate()
        .filter(|(i, line)| {
            let x = (i / down) * right % line_length;
            (line.chars().nth(x).unwrap() == '#') && i % down == 0
        })
        .count() as i64;
    return res;
}

fn solve_1(input: &str) -> i64 {
    return solve(input, 3, 1);
}

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn solve_2(input: &str) -> i64 {
    return SLOPES
        .iter()
        .map(|(right, down)| solve(input, *right, *down))
        .product();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
