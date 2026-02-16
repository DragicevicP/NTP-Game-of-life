#[path = "common.rs"]
mod common;
pub use common::*;

#[path = "sequential.rs"]
mod sequential;

#[path = "parallel.rs"]
mod parallel;

use std::time::Instant;


pub fn mean_std(times: &[f64]) -> (f64, f64) {
    let n = times.len() as f64;
    let mean = times.iter().sum::<f64>() / n;
    let var = times.iter().map(|t| (t - mean) * (t - mean)).sum::<f64>() / n;
    (mean, var.sqrt())
}


pub fn measure_time_sequential(base: &[Cell], rows: usize, cols: usize, iters: usize, print_time: bool, rep_idx: usize) -> f64 {
    let mut grid = base.to_vec();
    let start = Instant::now();

    for _ in 0..iters {
        grid = sequential::new_generation_sequential(&grid, rows, cols);
    }

    let _checksum: u64 = grid.iter().fold(0u64, |acc, &x| acc + x as u64);

    let t = start.elapsed().as_secs_f64();
    if print_time {
        println!("[SEQ] rep {} -> {:.6} s", rep_idx, t);
    }
    t
}


pub fn measure_time_parallel(base: &[Cell], rows: usize, cols: usize, iters: usize, p: usize, print_time: bool, rep_idx: usize) -> f64 {
    let mut grid = base.to_vec();
    let start = Instant::now();

    for _ in 0..iters {
        grid = parallel::new_generation_parallel(&grid, rows, cols, p);
    }

    let _checksum: u64 = grid.iter().fold(0u64, |acc, &x| acc + x as u64);

    let t = start.elapsed().as_secs_f64();
    if print_time {
        println!("[PAR p={}] rep {} -> {:.6} s", p, rep_idx, t);
    }
    t
}