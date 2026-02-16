#[path = "../parallel.rs"]
mod parallel;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    parallel::run_simulation(22,22,10, threads, "output/par_states.txt", 0.2);
    println!("Execution time: {:.4} seconds", start.elapsed().as_secs_f64());
}