use crate::helpers;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_OPS: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const EXAMPLE_OP: &str = "acc -99";

    #[test]
    fn test_parse_op() {
        assert_eq!(
            parse_op(EXAMPLE_OP),
            Op {
                code: OpCode::ACC,
                arg: -99,
                is_executed: false,
            },
        );
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE_OPS), 5)
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE_OPS), 8);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpCode {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Op {
    code: OpCode,
    arg: i32,
    is_executed: bool,
}

fn parse_op(input: &str) -> Op {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) (.+)").unwrap();
    }
    let cap = RE.captures(input).unwrap();
    let code = match &cap[1] {
        "acc" => OpCode::ACC,
        "jmp" => OpCode::JMP,
        "nop" => OpCode::NOP,
        _ => panic!(),
    };
    let arg = cap[2].parse::<i32>().unwrap();
    return Op {
        code: code,
        arg: arg,
        is_executed: false,
    };
}

fn parse_input(input: &str) -> Vec<Op> {
    return input.split("\n").map(parse_op).collect();
}

fn solve(ops: &mut Vec<Op>) -> (bool, i32) {
    let mut op_idx = 0;
    let mut acc = 0;

    while op_idx < ops.len() {
        if ops[op_idx].is_executed {
            return (false, acc);
        }
        ops[op_idx].is_executed = true;
        let op = &ops[op_idx];
        match op.code {
            OpCode::ACC => {
                acc += op.arg;
                op_idx += 1;
            }
            OpCode::JMP => op_idx = ((op_idx as i32) + op.arg) as usize,
            OpCode::NOP => op_idx += 1,
        }
    }
    return (true, acc);
}

fn solve_1(input: &str) -> i32 {
    let mut ops = parse_input(input);
    return solve(&mut ops).1;
}

fn solve_2(input: &str) -> i32 {
    let ops = parse_input(input);
    for (idx, op) in ops
        .iter()
        .enumerate()
        .filter(|(_, op)| op.code == OpCode::JMP || op.code == OpCode::NOP)
    {
        let mut tweaked_ops = ops.clone();
        if op.code == OpCode::JMP {
            tweaked_ops[idx].code = OpCode::NOP;
        } else {
            tweaked_ops[idx].code = OpCode::JMP;
        }
        let (did_finish, res) = solve(&mut tweaked_ops);
        if did_finish {
            return res;
        }
    }
    panic!();
}

pub fn run(input: &str) {
    helpers::run_benchmarked(|| println!("{}", solve_1(input)));
    helpers::run_benchmarked(|| println!("{}", solve_2(input)));
}
