use rand::Rng;
use std::io::{self, Write};

pub type Cell = u8;
pub type Grid = Vec<Cell>;

#[inline]
pub fn idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

pub fn create_random_grid(rows: usize, cols: usize, alive_probability: f64) -> Grid {
    let mut rng = rand::thread_rng();
    let mut grid = vec![0u8; rows * cols];

    for r in 0..rows {
        for c in 0..cols {
            let alive: bool =  rng.gen_bool(alive_probability);
            grid[idx(r, c, cols)] = if alive { 1 } else { 0 };
        }
    }

    grid
}

pub fn count_alive_neighbors( grid: &[Cell], rows: usize, cols: usize, row: usize, col: usize,) -> u8 {
    let mut alive = 0u8;

    for neighbor_rows in [-1i32, 0, 1] {
        for neighbor_cols in [-1i32, 0, 1] {
            if neighbor_rows == 0 && neighbor_cols == 0 {
                continue;
            }

            let r = row as i32 + neighbor_rows;
            let c = col as i32 + neighbor_cols;

            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                alive += grid[idx(r as usize, c as usize, cols)];
            }
        }
    }

    alive
}

pub fn next_cell_state(current: Cell, alive_neighbors: u8) -> Cell {
    if current == 1 {
        if alive_neighbors == 2 || alive_neighbors == 3 { 1 } else { 0 }
    } else {
        if alive_neighbors == 3 { 1 } else { 0 }
    }
}

pub fn save_grid<W: Write>( mut out: W, grid: &[Cell], rows: usize, cols: usize, iteration: usize,) -> io::Result<()> {
    writeln!(out, "Iteration {}", iteration)?;
    for r in 0..rows {
        for c in 0..cols {
            write!(out, "{}", grid[idx(r, c, cols)])?;
        }
        writeln!(out)?;
    }
    writeln!(out)?;
    Ok(())
}