#[path = "common.rs"]
mod common;
use common::*;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use std::thread;


pub fn new_generation_parallel(grid: &[Cell], rows: usize, cols: usize, threads_count: usize) -> Grid {
    let threads_count = threads_count.max(1).min(rows);

    let shared = Arc::new(grid.to_vec()); 
    let chunk = rows / threads_count;

    let mut handles = Vec::with_capacity(threads_count);
    let mut start_row = 0usize;

    for i in 0..threads_count {
        let end_row = if i == threads_count - 1 { rows } else { start_row + chunk };
        let grid_clone = Arc::clone(&shared);

        let handle = thread::spawn(move || -> (usize, Vec<Cell>) {
            let mut part = vec![0u8; (end_row - start_row) * cols];

            for r in start_row..end_row {
                for c in 0..cols {
                    let neighbors = count_alive_neighbors(&grid_clone, rows, cols, r, c);
                    let current = grid_clone[idx(r, c, cols)];
                    let next = next_cell_state(current, neighbors);

                    let local_r = r - start_row;
                    part[local_r * cols + c] = next;
                }
            }

            (start_row, part)
        });

        handles.push(handle);
        start_row = end_row;
    }

    let mut new_grid = vec![0u8; rows * cols];

    for h in handles {
        let (start, part) = h.join().unwrap();
        let offset = start * cols;
        new_grid[offset..offset + part.len()].copy_from_slice(&part);
    }

    new_grid
}

pub fn run_simulation(rows: usize, cols: usize, iterations: usize, threads: usize, output_path: &str, alive_probability: f64,) {
    let mut grid = create_random_grid(rows, cols, alive_probability);

    let file = File::create(output_path).unwrap();
    let mut writer = BufWriter::new(file);

    save_grid(&mut writer, &grid, rows, cols, 0).unwrap();

    for it in 1..=iterations {
        grid = new_generation_parallel(&grid, rows, cols, threads);
        save_grid(&mut writer, &grid, rows, cols, it).unwrap();
    }
}

