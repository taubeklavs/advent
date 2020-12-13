use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input(EXAMPLE_1),
            (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)])
        );
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_1), 295);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_1), 1068781);
    }
}

fn parse_input(input: &str) -> (u64, Vec<(u64, u64)>) {
    let mut splat = input.split("\n");
    let timestamp = splat.next().unwrap().parse::<u64>().unwrap();
    let bus_ids = splat
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, id)| match id.parse::<u64>() {
            Ok(parsed_id) => Some((i as u64, parsed_id)),
            _ => None,
        })
        .collect();
    return (timestamp, bus_ids);
}

fn solve_1(input: &str) -> u64 {
    let (timestamp, bus_ids) = parse_input(input);
    let mut closest_departure = u64::MAX;
    let mut closest_bus_id = 0;
    for (_, bus_id) in bus_ids {
        let next_departure = bus_id - timestamp % bus_id;
        if next_departure < closest_departure {
            closest_departure = next_departure;
            closest_bus_id = bus_id;
        }
    }
    return closest_departure * closest_bus_id;
}

fn solve_2(input: &str) -> u64 {
    let (_, bus_ids) = parse_input(input);
    let mut timestamp = 0;
    let mut step = 1;
    for (i, bus_id) in bus_ids {
        while (timestamp + i) % bus_id != 0 {
            timestamp += step;
        }
        step *= bus_id;
    }
    return timestamp;
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
