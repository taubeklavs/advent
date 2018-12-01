use std::io;
use std::env;

mod y2017;
mod y2018;

fn main() {
    let day = env::args().nth(1).unwrap();

    let mut input = String::new();
    while (io::stdin().read_line(&mut input).unwrap_or_default() > 0) {};
    let trimmed_input = input.trim();

    let day_fn = match day.as_ref() {
        "201701" => y2017::d1::run,
        "201801" => y2018::d1::run,
        _ => panic!("Day {:?} has not been implemented yet.", day),
    };
    day_fn(trimmed_input);
}
