use std::time::Instant;

pub fn run_benchmarked<F: FnOnce()>(f: F) {
    let start = Instant::now();
    f();
    println!("Took {:.2?} to run", start.elapsed());
}
