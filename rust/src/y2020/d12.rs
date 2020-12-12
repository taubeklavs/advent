use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_turn_ship() {
        assert_eq!(turn_ship('L', 90, 0), 3);
        assert_eq!(turn_ship('L', 180, 0), 2);
        assert_eq!(turn_ship('L', 270, 0), 1);
        assert_eq!(turn_ship('L', 360, 0), 0);
        assert_eq!(turn_ship('R', 90, 0), 1);
        assert_eq!(turn_ship('R', 180, 0), 2);
        assert_eq!(turn_ship('R', 270, 0), 3);
        assert_eq!(turn_ship('R', 360, 0), 0);
    }

    #[test]
    fn test_rotate_waypoint() {
        assert_eq!(rotate_waypoint('R', 90, 10, -4), (4, 10));
        assert_eq!(rotate_waypoint('R', 90, 4, 10), (-10, 4));
        assert_eq!(rotate_waypoint('R', 90, -10, 4), (-4, -10));
        assert_eq!(rotate_waypoint('R', 90, -4, -10), (10, -4));
    }
    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_1), 25);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_1), 286);
    }
}

fn parse_input(input: &str) -> Vec<(char, i32)> {
    return input
        .split("\n")
        .map(|line| {
            let mut line_chars = line.chars();
            return (
                line_chars.next().unwrap(),
                line_chars.as_str().parse::<i32>().unwrap(),
            );
        })
        .collect();
}

const CARDINAL_DIRECTIONS: [char; 4] = ['E', 'S', 'W', 'N'];

fn turn_ship(turn_direction: char, degrees: i32, dir: usize) -> usize {
    let abs_turns = degrees / 90;
    let dir_turns = match turn_direction {
        'L' => abs_turns * -1,
        'R' => abs_turns,
        _ => panic!(),
    };
    return ((((dir as i32 + dir_turns) % 4) + 4) % 4) as usize;
}

fn go_in_direction(dir: &char, distance: i32, x: i32, y: i32) -> (i32, i32) {
    return match dir {
        'E' => (x + distance, y),
        'W' => (x - distance, y),
        'S' => (x, y + distance),
        'N' => (x, y - distance),
        _ => panic!(),
    };
}

fn solve_1(input: &str) -> i32 {
    let commands = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for (command, arg) in commands {
        match command {
            'N' | 'S' | 'E' | 'W' => (x, y) = go_in_direction(&command, arg, x, y),
            'L' | 'R' => dir = turn_ship(command, arg, dir),
            'F' => (x, y) = go_in_direction(&CARDINAL_DIRECTIONS[dir], arg, x, y),
            _ => panic!(),
        }
    }
    return x.abs() + y.abs();
}

fn go_to_waypoint(arg: i32, wx: i32, wy: i32, x: i32, y: i32) -> (i32, i32) {
    return (x + wx * arg, y + wy * arg);
}

fn rotate_waypoint(rotation_direction: char, degrees: i32, wx: i32, wy: i32) -> (i32, i32) {
    return match ((degrees / 90 % 4), rotation_direction) {
        (1, 'L') | (3, 'R') => (wy, wx * -1),
        (1, 'R') | (3, 'L') => (wy * -1, wx),
        (2, _) => (wx * -1, wy * -1),
        (0, _) => (wx, wy),
        _ => panic!(),
    };
}

fn solve_2(input: &str) -> i32 {
    let commands = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = -1;
    for (command, arg) in commands {
        match command {
            'N' | 'S' | 'E' | 'W' => (wx, wy) = go_in_direction(&command, arg, wx, wy),
            'L' | 'R' => (wx, wy) = rotate_waypoint(command, arg, wx, wy),
            'F' => (x, y) = go_to_waypoint(arg, wx, wy, x, y),
            _ => panic!(),
        }
    }
    return x.abs() + y.abs();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
