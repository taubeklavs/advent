use regex::Regex;
use std::collections::HashMap;

use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BAGS: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_BAGS_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    const EXAMPLE_BAG: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

    #[test]
    fn test_parse_bag() {
        assert_eq!(
            parse_bag(EXAMPLE_BAG),
            (
                "light red".to_string(),
                Some(
                    vec![
                        ("bright white".to_string(), 1),
                        ("muted yellow".to_string(), 2)
                    ]
                    .into_iter()
                    .collect()
                )
            )
        )
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_BAGS), 4)
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_BAGS), 32);
        assert_eq!(solve_2(EXAMPLE_BAGS_2), 126);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BagChild {
    name: String,
    cnt: i32,
}

fn parse_bag_children(input: &str) -> Option<HashMap<String, i32>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (.+) bag").unwrap();
    }
    let mut bag_children: HashMap<String, i32> = HashMap::new();

    for bag in input.split(", ") {
        match RE.captures(bag) {
            Some(cap) => bag_children.insert(cap[2].to_string(), cap[1].parse::<i32>().unwrap()),
            None => return None,
        };
    }
    return Some(bag_children);
}

fn parse_bag(input: &str) -> (String, Option<HashMap<String, i32>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) bags contain (.+)").unwrap();
    }
    let cap = RE.captures(input).unwrap();

    return (cap[1].to_string(), parse_bag_children(&cap[2]));
}

fn can_contain_gold_bag(
    bag_definitions: &HashMap<String, Option<HashMap<String, i32>>>,
    children: &Option<HashMap<String, i32>>,
) -> bool {
    if children.is_none() {
        return false;
    } else if children.as_ref().unwrap().get("shiny gold").is_some() {
        return true;
    } else {
        for child in children.as_ref().unwrap().keys() {
            if can_contain_gold_bag(bag_definitions, bag_definitions.get(child).unwrap()) {
                return true;
            }
        }
        return false;
    }
}

fn parse_bags(input: &str) -> HashMap<String, Option<HashMap<String, i32>>> {
    let mut bag_definitions: HashMap<String, Option<HashMap<String, i32>>> = HashMap::new();

    for bag in input.split("\n") {
        let (name, children) = parse_bag(bag);
        bag_definitions.insert(name, children);
    }
    return bag_definitions;
}

fn solve_1(input: &str) -> i32 {
    let bag_definitions = parse_bags(input);
    return bag_definitions
        .iter()
        .filter(|(_, v)| can_contain_gold_bag(&bag_definitions, v))
        .count() as i32;
}

fn count_bags(
    bag_definitions: &HashMap<String, Option<HashMap<String, i32>>>,
    current_bag: &String,
    multiplier: i32,
) -> i32 {
    let mut res = multiplier;
    let children = bag_definitions.get(current_bag).unwrap();
    if children.is_some() {
        for (child_name, child_cnt) in children.as_ref().unwrap() {
            res += count_bags(&bag_definitions, &child_name, child_cnt * multiplier);
        }
    }
    return res;
}

fn solve_2(input: &str) -> i32 {
    let bag_definitions = parse_bags(input);
    return count_bags(&bag_definitions, &"shiny gold".to_string(), 1) - 1;
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
