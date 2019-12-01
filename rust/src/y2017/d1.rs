#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_captcha_simple() {
        assert_eq!(reverse_captcha("1122"), 3);
        assert_eq!(reverse_captcha("1111"), 4);
        assert_eq!(reverse_captcha("1234"), 0);
        assert_eq!(reverse_captcha("91212129"), 9);
    }

    #[test]
    fn opposite_captcha_simple() {
        assert_eq!(opposite_captcha("1212"), 6);
        assert_eq!(opposite_captcha("1221"), 0);
        assert_eq!(opposite_captcha("123425"), 4);
        assert_eq!(opposite_captcha("123123"), 12);
        assert_eq!(opposite_captcha("12131415"), 4);
    }
}

fn parse_input(captcha: &str) -> Vec<u32> {
    let digits = captcha.chars().map(|x| x.to_digit(10).unwrap()).collect();
    return digits;
}

fn process_captcha(captcha: Vec<u32>, filter_fn: fn(usize, u32, &Vec<u32>) -> bool) -> u32 {
    let res: u32 = captcha
        .iter()
        .enumerate()
        .filter_map(|(n, &x)| {
            if filter_fn(n, x, &captcha) {
                Some(x)
            } else {
                None
            }
        })
        .sum();
    return res;
}

fn reverse_captcha_filter_fn(iteration: usize, value: u32, captcha: &Vec<u32>) -> bool {
    return (iteration + 1 < captcha.len()) && (value == captcha[iteration + 1]);
}

fn reverse_captcha(captcha: &str) -> u32 {
    let mut parsed_captcha: Vec<u32> = parse_input(captcha);
    parsed_captcha.push(parsed_captcha[0]);
    return process_captcha(parsed_captcha, reverse_captcha_filter_fn);
}

fn opposite_captcha_filter_fn(iteration: usize, value: u32, captcha: &Vec<u32>) -> bool {
    let index = (iteration + (captcha.len() / 2)) % (captcha.len());
    return captcha[index] == value;
}

fn opposite_captcha(captcha: &str) -> u32 {
    let parsed_captcha: Vec<u32> = parse_input(captcha);
    return process_captcha(parsed_captcha, opposite_captcha_filter_fn);
}

pub fn run(captcha: &str) {
    println!("{}", reverse_captcha(captcha));
    println!("{}", opposite_captcha(captcha));
}
