#[path = "../sequential.rs"]
mod sequential;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    sequential::run_simulation(10, 10, 10, "output/seq_states.txt", 0.2);
    println!("Execution time: {:.4} seconds", start.elapsed().as_secs_f64());
}