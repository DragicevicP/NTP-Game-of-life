use macroquad::prelude::*;
use std::fs;

fn parse_iterations(text: &str) -> Vec<Vec<Vec<u8>>> {
    let mut iters: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut current: Vec<Vec<u8>> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Iteration") {
            if !current.is_empty() {
                iters.push(current);
                current = Vec::new();
            }
            continue;
        }

      
        let row: Vec<u8> = line.bytes().filter_map(|b| match b {
                b'0' => Some(0),
                b'1' => Some(1),
                _ => None,
            }).collect();

        if !row.is_empty() {
            current.push(row);
        }
    }

    if !current.is_empty() {
        iters.push(current);
    }

    iters
}

#[macroquad::main("Game of Life Viewer")]
async fn main() {
  
    let path = "output/par_states.txt";
    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("File could not be found: {}", path));

    let iterations = parse_iterations(&content);
    if iterations.is_empty() {
        panic!("File is empty.");
    }

    let rows = iterations[0].len();
    let cols = iterations[0][0].len();

    let mut idx: usize = 0;
    let mut playing = false;
    let mut cell_size: f32 = 22.0; 
    let fps: f32 = 1.0;    
    let mut acc: f32 = 0.0;

    loop {
        let dt = get_frame_time();
        acc += dt;

    
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            playing = !playing;
        }
        if is_key_pressed(KeyCode::Right) {
            idx = (idx + 1).min(iterations.len() - 1);
        }
        if is_key_pressed(KeyCode::Left) {
            idx = idx.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::R) {
            idx = 0;
            playing = false;
            acc = 0.0;
        }
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            cell_size = (cell_size + 2.0).min(80.0);
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            cell_size = (cell_size - 2.0).max(4.0);
        }

        if playing && acc >= 1.0 / fps {
            acc = 0.0;
            if idx + 1 < iterations.len() {
                idx += 1;
            } else {
                playing = false; 
            }
        }
     
        clear_background(Color::from_rgba(18, 18, 22, 255));

        let alive = Color::from_rgba(80, 220, 140, 255);
        let dead = Color::from_rgba(35, 35, 45, 255);
        let grid = Color::from_rgba(55, 55, 70, 255);
        let text_col = Color::from_rgba(230, 230, 240, 255);
      
        let top_bar_h = 72.0;
        let padding = 18.0;

        let board_w = cols as f32 * cell_size;
        let board_h = rows as f32 * cell_size;

        let start_x = (screen_width() - board_w).max(0.0) * 0.5;
        let start_y = top_bar_h + ((screen_height() - top_bar_h - board_h).max(0.0) * 0.5);

        // info bar
        draw_text(
            &format!(
                "Iteration: {} / {}   |   Board: {} x {}   |   Mode: {}   |   Zoom: {}px",
                idx,
                iterations.len() - 1,
                rows,
                cols,
                if playing { "PLAY" } else { "PAUSE" },
                cell_size as i32
            ),
            padding,
            28.0,
            22.0,
            text_col,
        );

        draw_text(
            "Controls:  <-/-> step, Space play/pause, +/- zoom, R reset, Esc exit",
            padding,
            54.0,
            18.0,
            Color::from_rgba(190, 190, 205, 255),
        );

  
        let frame = &iterations[idx];

        for r in 0..rows {
            for c in 0..cols {
                let x = start_x + c as f32 * cell_size;
                let y = start_y + r as f32 * cell_size;

                let is_alive = frame[r][c] == 1;
                draw_rectangle(x, y, cell_size, cell_size, if is_alive { alive } else { dead });
                draw_rectangle_lines(x, y, cell_size, cell_size, 1.0, grid);
            }
        }
        next_frame().await;
    }
}