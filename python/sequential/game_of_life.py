import random
import time
from typing import List
from python.common.game_logic import (
    create_random_grid,
    count_alive_neighbors,
    save_grid
)

Grid = List[List[int]]

def new_generation(grid: Grid):

    rows = len(grid)
    cols = len(grid[0])

    new_grid = []

    for _ in range(rows):
        row = [] 
        for _ in range(cols):
            row.append(0)
        
        new_grid.append(row)

    
    for i in range(rows):
        for j in range(cols):
            alive_neighbors = count_alive_neighbors(grid, i, j)

            if grid[i][j] == 1:
                if alive_neighbors in (2, 3):
                    new_grid[i][j] = 1
            else:
                if alive_neighbors == 3:
                    new_grid[i][j] = 1

    return new_grid


def run_simulation(rows: int, cols: int, iterations: int, output_path: str):
 
    grid = create_random_grid(rows, cols)

    with open(output_path, "w") as file:
        save_grid(file, grid, 0)

        for it in range(1, iterations + 1):
            grid = new_generation(grid)
            save_grid(file, grid, it)


if __name__ == "__main__":
    start = time.perf_counter()

    run_simulation(rows=1000, cols=1000, iterations=100, output_path="python/output/seq_states.txt")

    end = time.perf_counter()

    print(f"Execution time: {end - start:.4f} seconds")
