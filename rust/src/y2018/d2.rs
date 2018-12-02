use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appears_2_or_3_times_simple() {
        assert_eq!(appears_2_or_3_times(&"abcdef".to_string()), (false, false));
        assert_eq!(appears_2_or_3_times(&"bababc".to_string()), (true, true));
        assert_eq!(appears_2_or_3_times(&"abbcde".to_string()), (true, false));
        assert_eq!(appears_2_or_3_times(&"abcccd".to_string()), (false, true));
        assert_eq!(appears_2_or_3_times(&"aabcdd".to_string()), (true, false));
        assert_eq!(appears_2_or_3_times(&"abcdee".to_string()), (true, false));
        assert_eq!(appears_2_or_3_times(&"ababab".to_string()), (false, true));
    }

    #[test]
    fn overlap_simple() {
        let input = ["abcde",
                     "fghij",
                     "klmno",
                     "pqrst",
                     "fguij",
                     "axcye",
                     "wvxyz"]
            .iter()
            .map(|line| line.to_string())
            .collect();
        assert_eq!(overlap(&input), Some("fgij".to_string()));
    }
}

fn parse_input(payload: &str) -> Vec<String> {
    return payload
        .to_string()
        .split("\n")
        .map(|line| line.to_string())
        .collect();
}

fn appears_2_or_3_times(line: &String) -> (bool, bool) {
    let mut appearances = HashMap::new();
    for c in line.chars() {
        let appearance = appearances.entry(c).or_insert(0);
        *appearance += 1;
    }
    return (appearances.values().find(|&val| *val == 2).is_some(),
            appearances.values().find(|&val| *val == 3).is_some());
}

fn checksum (lines: &Vec<String>) -> i32 {
    let mut twice = 0;
    let mut thrice = 0;
    for line in lines {
        let (twice_line, thrice_line) = appears_2_or_3_times(line);
        if twice_line { twice += 1}
        if thrice_line { thrice += 1}
    }
    return twice * thrice;
}

fn line_overlap (line1: &String, line2: &String) -> String {
    let mut overlap = String::new();
    for i in 0..line1.len() {
        if line1.chars().nth(i) == line2.chars().nth(i) {
            overlap.push(line1.chars().nth(i).unwrap());
        }
    }
    return overlap;
}

fn overlap (lines: &Vec<String>) -> Option<String> {
    for i in 0..lines.len() {
        for j in i..lines.len() {
            let line_overlap = line_overlap(&lines[i], &lines[j]);
            if line_overlap.len() == lines[i].len() - 1 {
                return Some(line_overlap);
            }
        }
    }
    return None;
}

pub fn run(payload: &str) {
    let parsed = parse_input(payload);
    println!("{:?}", checksum(&parsed));
    println!("{:?}", overlap(&parsed).unwrap());
}
