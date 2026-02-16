#[path = "../common_test.rs"]
mod common_test;
use common_test::*;

use std::fs::File;
use std::io::{BufWriter, Write};

const BASE_COLS: usize = 100;
const ITERS: usize = 100;
const REPS: usize = 30;

fn main() {
    let processes = [1usize, 2, 4, 8, 16];

    std::fs::create_dir_all("output").unwrap();
    let file = File::create("output/weak_scaling.csv").unwrap();
    let mut w = BufWriter::new(file);

    writeln!(w, "Processes,AvgTime(s),StdDev(s)").unwrap();
   
    for &p in &processes {
        let rows = 100 * p;
        let cols = BASE_COLS;

        let base = create_random_grid(rows, cols, 0.2);

        let mut par_times = Vec::with_capacity(REPS);
        for rep in 1..=REPS {
            par_times.push(measure_time_parallel(&base, rows, cols, ITERS, p, true, rep));
        }
        let (avg, std) = mean_std(&par_times);
        writeln!(w, "{},{:.12},{:.12}", p, avg, std).unwrap();
    }

    w.flush().unwrap();
    println!("Wrote output/weak_scaling.csv");
}