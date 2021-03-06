#![feature(destructuring_assignment)]

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
        "202002" => y2020::d2::run,
        "202003" => y2020::d3::run,
        "202004" => y2020::d4::run,
        "202005" => y2020::d5::run,
        "202006" => y2020::d6::run,
        "202007" => y2020::d7::run,
        "202008" => y2020::d8::run,
        "202009" => y2020::d9::run,
        "202010" => y2020::d10::run,
        "202011" => y2020::d11::run,
        "202012" => y2020::d12::run,
        "202013" => y2020::d13::run,
        "202014" => y2020::d14::run,
        _ => panic!("Day {:?} has not been implemented yet.", day),
    };
    day_fn(trimmed_input);
}
