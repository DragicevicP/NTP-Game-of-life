#[path = "../common_test.rs"]
mod common_test;
use common_test::*;

use std::fs::File;
use std::io::{BufWriter, Write};

const ROWS: usize = 1600;
const COLS: usize = 1600;
const ITERS: usize = 100;
const REPS: usize = 30;

fn main() {
    let base = create_random_grid(ROWS, COLS, 0.2);
    let processes = [1usize, 2, 4, 8, 16];

    std::fs::create_dir_all("output").unwrap();
    let file = File::create("output/strong_scaling.csv").unwrap();
    let mut w = BufWriter::new(file);

    writeln!(w, "Processes,AvgTime(s),StdDev(s)").unwrap();


    let mut seq_times = Vec::with_capacity(REPS);
    for rep in 1..=REPS {
        seq_times.push(measure_time_sequential(&base, ROWS, COLS, ITERS, true, rep));
    }
    let (seq_avg, seq_std) = mean_std(&seq_times);
    writeln!(w, "seq,{:.12},{:.12}", seq_avg, seq_std).unwrap();

   
    for &p in &processes {
        let mut par_times = Vec::with_capacity(REPS);
        for rep in 1..=REPS {
            par_times.push(measure_time_parallel(&base, ROWS, COLS, ITERS, p, true, rep));
        }
        let (avg, std) = mean_std(&par_times);
        writeln!(w, "{},{:.12},{:.12}", p, avg, std).unwrap();
    }

    w.flush().unwrap();
    println!("Wrote output/strong_scaling.csv");
}