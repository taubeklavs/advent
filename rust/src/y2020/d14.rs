use crate::helpers;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    const EXAMPLE_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("mask = 0010011010X1000100X101011X10010X1010"),
            (
                Entry::MASK,
                u64::MAX,
                "0010011010X1000100X101011X10010X1010"
            )
        );
        assert_eq!(
            parse_line("mem[57319] = 8001842"),
            (Entry::MEM, 57319, "8001842")
        );
    }

    #[test]
    fn test_apply_value_mask() {
        assert_eq!(
            apply_value_mask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            73
        );
        assert_eq!(
            apply_value_mask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            101
        );
        assert_eq!(
            apply_value_mask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            64
        );
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_1), 165);
    }

    #[test]
    fn test_apply_address_mask() {
        let addresses = apply_address_mask(26, "00000000000000000000000000000000X0XX");
        let address_set: HashSet<u64> =
            HashSet::from_iter((addresses as VecDeque<u64>).into_iter());
        assert_eq!(
            address_set,
            HashSet::from_iter(vec![16, 17, 18, 19, 24, 25, 26, 27])
        );
    }
}

#[derive(PartialEq, Debug)]
enum Entry {
    MASK,
    MEM,
}

fn parse_line(line: &str) -> (Entry, u64, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<entry>[[:alpha:]]+)(?:\[(?P<address>[[:digit:]]+)\])* = (?P<value>[[:alnum:]]+)"
        )
        .unwrap();
    }
    let cap = RE.captures(line).unwrap();
    let entry = match cap.name("entry").unwrap().as_str() {
        "mask" => Entry::MASK,
        "mem" => Entry::MEM,
        _ => panic!(),
    };
    let address = match cap.name("address") {
        Some(a) => a.as_str().parse::<u64>().unwrap(),
        _ => u64::MAX,
    };
    return (entry, address, cap.name("value").unwrap().as_str());
}

fn apply_value_mask(value: u64, mask: &str) -> u64 {
    let mut bit = 0;
    let mut value = value;
    for c in mask.chars().rev() {
        match c {
            '1' => value |= 1 << bit,
            '0' => value &= !(1 << bit),
            _ => (),
        }
        bit += 1;
    }
    return value;
}

fn solve_1(input: &str) -> u64 {
    let lines = input.split("\n").map(parse_line);
    let mut mask: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for (entry, address, value) in lines {
        match entry {
            Entry::MASK => {
                mask = value;
            }
            Entry::MEM => {
                memory.insert(
                    address,
                    apply_value_mask(value.parse::<u64>().unwrap(), mask),
                );
            }
        }
    }
    return memory.values().sum();
}

fn apply_address_mask(address: u64, mask: &str) -> VecDeque<u64> {
    let mut bit = 0;
    let mut address = address;
    let mut x_bits = vec![];
    for c in mask.chars().rev() {
        match c {
            '1' => {
                address |= 1 << bit;
            }
            'X' => {
                x_bits.push(bit);
            }
            _ => (),
        }
        bit += 1;
    }
    let mut addresses = VecDeque::new();
    addresses.push_back(address);
    for x_bit in x_bits {
        for _ in 0..addresses.len() {
            let address = addresses.pop_front().unwrap();
            addresses.push_back(address | (1 << x_bit));
            addresses.push_back(address & !(1 << x_bit));
        }
    }
    return addresses;
}

fn solve_2(input: &str) -> u64 {
    let lines = input.split("\n").map(parse_line);
    let mut mask: &str = "000000000000000000000000000000000000";
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for (entry, address, value) in lines {
        match entry {
            Entry::MASK => {
                mask = value;
            }
            Entry::MEM => {
                let addresses = apply_address_mask(address, mask);
                for address in addresses {
                    memory.insert(address, value.parse::<u64>().unwrap());
                }
            }
        }
    }
    return memory.values().sum();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
