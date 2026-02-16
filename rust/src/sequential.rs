#[path = "common.rs"]
mod common;
use common::*;
use std::fs::File;
use std::io::BufWriter;

pub fn new_generation_sequential(grid: &[Cell], rows: usize, cols: usize) -> Grid {
    let mut new_grid: Vec<u8> = vec![0u8; rows * cols];

    for r in 0..rows {
        for c in 0..cols {
            let neighbors = count_alive_neighbors(grid, rows, cols, r, c);
            let current = grid[idx(r, c, cols)];
            new_grid[idx(r, c, cols)] = next_cell_state(current, neighbors);
        }
    }

    new_grid
}

pub fn run_simulation(rows: usize, cols: usize, iterations: usize, output_path: &str, alive_probability: f64) {
    let mut grid = create_random_grid(rows, cols, alive_probability);

    let file = File::create(output_path).unwrap();
    let mut writer = BufWriter::new(file);

    save_grid(&mut writer, &grid, rows, cols, 0).unwrap();

    for it in 1..=iterations {
        grid = new_generation_sequential(&grid, rows, cols);
        save_grid(&mut writer, &grid, rows, cols, it).unwrap();
    }
}

