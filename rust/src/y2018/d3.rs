use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_claim() {
        assert_eq!(parse_claim("#1 @ 1,3: 4x4"),
                   Claim { id:1,
                           offset_x: 1,
                           offset_y: 3,
                           size_x: 4,
                           size_y: 4 });
        assert_eq!(parse_claim("#2 @ 3,1: 4x4"),
                   Claim { id:2,
                           offset_x: 3,
                           offset_y: 1,
                           size_x: 4,
                           size_y: 4 });
        assert_eq!(parse_claim("#3 @ 5,5: 2x2"),
                   Claim { id:3,
                           offset_x: 5,
                           offset_y: 5,
                           size_x: 2,
                           size_y: 2 });
    }

    #[test]
    fn test_index_2d() {
        assert_eq!(index_2d(0, 0, 0, 0, 300), 0);
        assert_eq!(index_2d(2, 1, 0, 0, 4), 3);
        assert_eq!(index_2d(0, 0, 1, 0, 2), 2);
    }

    #[test]
    fn overlap_simple() {
        assert_eq!(overlap("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
    }
}

const FABRIC_WIDTH: i32 = 1000;

#[derive(Debug)]
#[derive(PartialEq)]
struct Claim {
    id: i32,
    offset_x: i32,
    offset_y: i32,
    size_x: i32,
    size_y: i32,
}

fn index_2d(x: i32, x_offset: i32, y_offset: i32, y: i32, width: i32) -> usize {
    return (x + x_offset + (y + y_offset) * width) as usize;
}

fn parse_claim(claim: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let cap = RE.captures(claim).unwrap();
    return Claim { id: cap[1].parse::<i32>().unwrap(),
                   offset_x: cap[2].parse::<i32>().unwrap(),
                   offset_y: cap[3].parse::<i32>().unwrap(),
                   size_x: cap[4].parse::<i32>().unwrap(),
                   size_y: cap[5].parse::<i32>().unwrap() };
}

fn apply_claim(mut fabric: (Vec<(i32, i32)>, HashMap<i32, bool>),
               claim: &str) -> (Vec<(i32, i32)>, HashMap<i32, bool>) {
    let claim = parse_claim(claim);
    for x in 0..claim.size_x {
        for y in 0..claim.size_y {
            if let Some(square_inch) =
                fabric.0.get_mut(index_2d(x,
                                          claim.offset_x,
                                          y,
                                          claim.offset_y,
                                          FABRIC_WIDTH)) {
                    if square_inch.0 >= 1 {
                        fabric.1.insert(square_inch.1, true);
                        fabric.1.insert(claim.id, true);
                    }
                    if square_inch.0 == 0 &&
                        fabric.1.contains_key(&claim.id) == false {
                            fabric.1.insert(claim.id, false);
                    }
                    *square_inch = (square_inch.0 + 1 , claim.id);
            }
        }
    }
    return fabric;
}

fn overlap(claims: &str) -> i32 {
    return claims
        .to_string()
        .split("\n")
        .fold((vec![(0, 0); (FABRIC_WIDTH * FABRIC_WIDTH) as usize],
               HashMap::new()),
              |fabric, claim| apply_claim(fabric, claim))
        .0
        .iter()
        .filter(|square_inch| square_inch.0 > 1)
        .count() as i32;
}

fn outlier(claims: &str) -> i32 {
    return claims
        .to_string()
        .split("\n")
        .fold((vec![(0, 0); (FABRIC_WIDTH * FABRIC_WIDTH) as usize],
               HashMap::new()),
              |fabric, claim| apply_claim(fabric, claim))
        .1
        .iter()
        .find(|(_, &overlapping)| overlapping == false)
        .unwrap()
        .0
        .clone();
}

pub fn run(claims: &str) {
    println!("{:?}", overlap(claims));
    println!("{:?}", outlier(claims));
}
