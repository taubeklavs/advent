use regex::Regex;
use std::collections::BTreeSet;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PASSPORT: &str =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";

    const EXAMPLE_INPUT: &str =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929
hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm
hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_validate_passport() {
        assert_eq!(validate_passport(EXAMPLE_PASSPORT, false), true)
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_validate_field() {
        assert_eq!(validate_field("byr", "1984"), true)
    }

    #[test]
    fn test_validate_hgt() {
        assert_eq!(validate_hgt("193cm"), true);
        assert_eq!(validate_hgt("194cm"), false);
        assert_eq!(validate_hgt("194"), false);
    }

    #[test]
    fn test_validate_hcl() {
        assert_eq!(validate_hcl("#123456"), true);
        assert_eq!(validate_hcl("#12345z"), false);
        assert_eq!(validate_hcl("#1234566"), false);
    }

    #[test]
    fn test_validate_ecl() {
        assert_eq!(validate_ecl("amb"), true);
        assert_eq!(validate_ecl("lamb"), false);
    }

    #[test]
    fn test_validate_pid() {
        assert_eq!(validate_pid("999999999"), true);
        assert_eq!(validate_pid("9999999990"), false);
    }

    const EXAMPLE_INVALID_PASSPORTS: &str =
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946
hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007";

    const EXAMPLE_VALID_PASSPORTS: &str =
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f
eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022
iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_INVALID_PASSPORTS), 0);
        assert_eq!(solve_2(EXAMPLE_VALID_PASSPORTS), 4);
    }
}

fn validate_year(value: &str, min: &str, max: &str) -> bool {
    return value.len() == 4 && value >= min && value <= max;
}

fn validate_hgt(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([[:digit:]]+)(in|cm)").unwrap();
    }
    let cap = match RE.captures(value) {
        Some(cap) => cap,
        None => return false,
    };
    let val = &cap[1];
    let unit = &cap[2];
    if unit == "cm" {
        return val >= "150" && val <= "193";
    } else if unit == "in" {
        return val >= "59" && val <= "76";
    }
    return false;
}

fn validate_hcl(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    return RE.is_match(value);
}

fn validate_ecl(value: &str) -> bool {
    lazy_static! {
        static ref VALID_ECLS: BTreeSet<&'static str> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .collect();
    }
    return VALID_ECLS.contains(value);
}

fn validate_pid(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    return RE.is_match(value);
}

fn validate_field(key: &str, value: &str) -> bool {
    return match key {
        "byr" => validate_year(value, "1920", "2002"),
        "iyr" => validate_year(value, "2010", "2020"),
        "eyr" => validate_year(value, "2020", "2030"),
        "hgt" => validate_hgt(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        "cid" => true,
        _ => panic!(),
    };
}

fn validate_passport(passport: &str, should_validate_fields: bool) -> bool {
    lazy_static! {
        static ref REQUIRED_FIELDS: BTreeSet<&'static str> =
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();
    }
    let mut missing_fields = REQUIRED_FIELDS.clone();
    let fields = passport.split(" ");
    for field in fields {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([[:alpha:]]+):(.+)").unwrap();
        }
        let cap = RE.captures(field).unwrap();
        missing_fields.remove(&cap[1]);
        if should_validate_fields && !validate_field(&cap[1], &cap[2]) {
            return false;
        }
    }
    return missing_fields.is_empty();
}

fn solve(input: &str, should_validate_fields: bool) -> i32 {
    return input
        .split("\n")
        .filter(|passport| validate_passport(passport, should_validate_fields))
        .count() as i32;
}

fn solve_1(input: &str) -> i32 {
    return solve(input, false);
}

fn solve_2(input: &str) -> i32 {
    return solve(input, true);
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
