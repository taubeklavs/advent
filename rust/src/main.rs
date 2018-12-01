use std::io;
use std::env;

mod y2017;

fn main() {
    let day = env::args().nth(1).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read input");
    let trimmed_input = input.trim();

    let day_fn = match day.as_ref() {
        "201701" => y2017::d1::run,
        _ => panic!("Day {:?} has not been implemented yet.", day),
    };
    day_fn(trimmed_input);
}
