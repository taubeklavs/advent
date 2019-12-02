#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operate() {
        assert_eq!(
            operate([1, 0, 0, 0, 99].to_vec(), 0, 0),
            [2, 0, 0, 0, 99].to_vec()
        );
        assert_eq!(
            operate([2, 3, 0, 3, 99].to_vec(), 3, 0),
            [2, 3, 0, 6, 99].to_vec()
        );
        assert_eq!(
            operate([2, 4, 4, 5, 99, 0].to_vec(), 4, 4),
            [2, 4, 4, 5, 99, 9801].to_vec()
        );
        assert_eq!(
            operate([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec(), 1, 1),
            [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec()
        );
    }
}

fn operate(mut intcode: Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    intcode[1] = noun;
    intcode[2] = verb;
    let mut pos = 0 as usize;
    while pos < intcode.len() - 3 {
        let operation = intcode[pos];
        let p1 = intcode[pos + 1] as usize;
        let p2 = intcode[pos + 2] as usize;
        let p3 = intcode[pos + 3] as usize;
        if operation == 1 {
            intcode[p3] = intcode[p1] + intcode[p2];
        } else if operation == 2 {
            intcode[p3] = intcode[p1] * intcode[p2];
        }
        pos += 4;
    }
    return intcode;
}

fn operate_p1(intcode: Vec<i32>) -> i32 {
    return operate(intcode, 12, 2)[0];
}

const REQUIRED_RESULT: i32 = 19690720;

fn operate_p2(intcode: Vec<i32>) -> i32 {
    let initial_operation = operate(intcode.clone(), 0, 0)[0];
    let noun_step = operate(intcode, 1, 0)[0] - initial_operation;
    let noun = REQUIRED_RESULT / noun_step - 1;
    let verb = (REQUIRED_RESULT - initial_operation) % noun_step;
    return 100 * noun + verb;
}
fn parse_input(intcode: &str) -> Vec<i32> {
    return intcode
        .to_string()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}
pub fn run(intcode: &str) {
    let parsed = parse_input(intcode);
    println!("{:?}", operate_p1(parsed.clone()));
    println!("{:?}", operate_p2(parsed));
}
