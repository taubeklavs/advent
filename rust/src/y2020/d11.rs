use std::cmp;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_1), 37);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_1), 26);
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    return input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
}

fn adjacent_occupants(x: usize, y: usize, seats: &Vec<Vec<char>>) -> i32 {
    let mut adjacent_occupants = 0;
    let min_x = x.checked_sub(1).unwrap_or(0);
    let max_x = cmp::min(seats.len(), x + 2);
    let min_y = y.checked_sub(1).unwrap_or(0);
    let max_y = cmp::min(seats[0].len(), y + 2);
    for ax in min_x..max_x {
        for ay in min_y..max_y {
            if seats[ax][ay] == '#' && !(ax == x && ay == y) {
                adjacent_occupants += 1;
            }
        }
    }
    return adjacent_occupants;
}

fn solve(
    input: &str,
    encumbering_occupant_count_fn: fn(usize, usize, &Vec<Vec<char>>) -> i32,
    tolerance: i32,
) -> i32 {
    let mut seats: Vec<Vec<char>> = parse_input(input);
    let mut anything_changed = false;
    let mut occupied_seats = 0;
    loop {
        let mut next_seats = seats.clone();
        for x in 0..seats.len() {
            for y in 0..seats[x].len() {
                let encumbering_occupants = encumbering_occupant_count_fn(x, y, &seats);
                if seats[x][y] == 'L' && encumbering_occupants == 0 {
                    anything_changed = true;
                    next_seats[x][y] = '#';
                    occupied_seats += 1;
                } else if seats[x][y] == '#' && encumbering_occupants >= tolerance {
                    anything_changed = true;
                    next_seats[x][y] = 'L';
                    occupied_seats -= 1;
                }
            }
        }
        if !anything_changed {
            break;
        } else {
            seats = next_seats;
            anything_changed = false;
        }
    }
    return occupied_seats;
}

static DIRECTIONS: &'static [(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn calc_idx(idx: usize, dir: i32, len: usize) -> Option<usize> {
    if dir.is_positive() {
        let new_idx = idx + 1;
        if new_idx >= len {
            return None;
        } else {
            return Some(new_idx);
        }
    } else if dir.is_negative() {
        return idx.checked_sub(1);
    } else {
        return Some(idx);
    }
}

fn visible_occupants(x: usize, y: usize, seats: &Vec<Vec<char>>) -> i32 {
    let mut visible_occupants = 0;
    for (x_dir, y_dir) in DIRECTIONS {
        let mut vx = x;
        let mut vy = y;
        loop {
            match calc_idx(vx, *x_dir, seats.len()) {
                Some(new_vx) => vx = new_vx,
                None => break,
            }
            match calc_idx(vy, *y_dir, seats[0].len()) {
                Some(new_vy) => vy = new_vy,
                None => break,
            }
            match seats[vx][vy] {
                '#' => {
                    visible_occupants += 1;
                    break;
                }
                'L' => break,
                _ => (),
            }
        }
    }
    return visible_occupants;
}

fn solve_1(input: &str) -> i32 {
    return solve(input, adjacent_occupants, 4);
}

fn solve_2(input: &str) -> i32 {
    return solve(input, visible_occupants, 5);
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
