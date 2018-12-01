use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_frequency_change(changes: &[i32], expected: i32) {
        assert_eq!(frequency_change(&changes.to_vec()), expected);
    }

    #[test]
    fn frequency_change_simple() {
        assert_frequency_change(&[1, -2, 3, 1], 3);
        assert_frequency_change(&[1, 1, 1], 3);
        assert_frequency_change(&[1, 1, -2], 0);
        assert_frequency_change(&[-1, -2, -3], -6);
    }

    fn assert_first_reached_twice(changes: &[i32], expected: i32) {
        assert_eq!(first_reached_twice(&changes.to_vec()), expected);
    }

    #[test]
    fn first_reached_twice_simple() {
        assert_first_reached_twice(&[1, -1], 0);
        assert_first_reached_twice(&[3, 3, 4, -2, -4], 10);
        assert_first_reached_twice(&[-6, 3, 8, 5, -6], 5);
        assert_first_reached_twice(&[7, 7, -2, -7, -4], 14);
    }
}

fn parse_input(changes: &str) -> Vec<i32> {
    return changes
        .to_string()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn frequency_change(changes: &Vec<i32>) -> i32 {
    return changes.iter().sum();
}

fn first_reached_twice(changes: &Vec<i32>) -> i32 {
    let mut reached = HashSet::new();
    let mut sum = 0;
    let mut index = 0;
    loop {
        if reached.contains(&sum) {
            return sum;
        } else {
            reached.insert(sum);
        }
        sum += changes[index % changes.len()];
        index += 1;
    }
}

pub fn run(changes: &str) {
    let parsed = parse_input(changes);
    println!("{:?}", frequency_change(&parsed));
    println!("{:?}", first_reached_twice(&parsed));
}
