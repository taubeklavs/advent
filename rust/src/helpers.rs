use std::time::Instant;

pub fn run_benchmarked<F: FnOnce() -> i32>(f: F) {
    let start = Instant::now();
    let res = f();
    println!("Took {:.2?} to run", start.elapsed());
    println!("{}", res);
}
