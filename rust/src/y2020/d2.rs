use regex::Regex;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1-3 a: abcde"),
            Password {
                arg_1: 1,
                arg_2: 3,
                symbol: 'a',
                text: "abcde".to_string()
            }
        )
    }

    #[test]
    fn test_is_valid_password_1() {
        assert_eq!(is_valid_password_1("1-3 a: abcde"), true);
        assert_eq!(is_valid_password_1("1-3 b: cdefg"), false);
        assert_eq!(is_valid_password_1("2-9 c: ccccccccc"), true);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}

#[derive(Debug, PartialEq)]
struct Password {
    arg_1: i32,
    arg_2: i32,
    symbol: char,
    text: String,
}

fn parse_line(line: &str) -> Password {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    return Password {
        arg_1: cap[1].parse::<i32>().unwrap(),
        arg_2: cap[2].parse::<i32>().unwrap(),
        symbol: cap[3].parse::<char>().unwrap(),
        text: cap[4].to_string(),
    };
}

fn is_valid_password_1(line: &str) -> bool {
    let password = parse_line(line);
    let symbol_occurences = password
        .text
        .chars()
        .filter(|c| c == &password.symbol)
        .count() as i32;
    if password.arg_1 <= symbol_occurences && symbol_occurences <= password.arg_2 {
        return true;
    } else {
        return false;
    }
}

fn is_valid_password_2(line: &str) -> bool {
    let password = parse_line(line);
    if (password
        .text
        .chars()
        .nth(password.arg_1 as usize - 1)
        .unwrap()
        == password.symbol)
        ^ (password
            .text
            .chars()
            .nth(password.arg_2 as usize - 1)
            .unwrap()
            == password.symbol)
    {
        return true;
    } else {
        return false;
    }
}
fn solve_1(input: &str) -> i32 {
    return input
        .split("\n")
        .filter(|password| is_valid_password_1(password))
        .count() as i32;
}

fn solve_2(input: &str) -> i32 {
    return input
        .split("\n")
        .filter(|password| is_valid_password_2(password))
        .count() as i32;
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
