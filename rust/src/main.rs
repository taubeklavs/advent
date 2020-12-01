#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;

mod y2017;
mod y2018;
mod y2019;
mod y2020;

mod helpers;

fn main() {
    let day = env::args().nth(1).unwrap();

    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap_or_default() > 0 {}
    let trimmed_input = input.trim();

    let day_fn = match day.as_ref() {
        "201701" => y2017::d1::run,
        "201801" => y2018::d1::run,
        "201802" => y2018::d2::run,
        "201803" => y2018::d3::run,
        "201901" => y2019::d1::run,
        "201902" => y2019::d2::run,
        "202001" => y2020::d1::run,
        _ => panic!("Day {:?} has not been implemented yet.", day),
    };
    day_fn(trimmed_input);
}
