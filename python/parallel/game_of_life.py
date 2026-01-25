import random
import time
from typing import List, Tuple
import multiprocessing as mp
from typing import Optional
from typing import Any



from python.common.game_logic import (
    create_random_grid,
    count_alive_neighbors,
    save_grid
)

Grid = List[List[int]]

GLOBAL_GRID = None

def init_worker(grid: Grid):
    global GLOBAL_GRID
    GLOBAL_GRID = grid

def new_generation_parallel(grid: Grid, pool: Any, processes: int):
    rows = len(grid)
    chunk_size = rows // processes
    tasks = []

    start = 0
    for i in range(processes):
        end = start + chunk_size
        if i == processes - 1:
            end = rows
        tasks.append((start, end))
        start = end

    results = pool.map(compute_rows, tasks)

    new_grid = []
    for part in results:
        new_grid.extend(part)

    return new_grid


def compute_rows(args: Tuple[int, int]):
    start_row, end_row = args
    grid = GLOBAL_GRID

    cols = len(grid[0])

    result = []

    for i in range(start_row, end_row):
        new_row = [0] * cols
        for j in range(cols):
            alive_neighbors = count_alive_neighbors(grid, i, j)

            if grid[i][j] == 1:
                if alive_neighbors in (2, 3):
                    new_row[j] = 1
            else:
                if alive_neighbors == 3:
                    new_row[j] = 1

        result.append(new_row)

    return result
    

def run_simulation(rows: int, cols: int, iterations: int, processes: int, output_path: Optional[str] = None):
    grid = create_random_grid(rows, cols)

    if output_path is not None:
        
        with open(output_path, "w") as file:
            save_grid(file, grid, 0)

            with mp.Pool(processes=processes, initializer=init_worker, initargs=(grid,)) as pool:
                for it in range(1, iterations + 1):
                    grid = new_generation_parallel(grid, pool, processes)
                    init_worker(grid)
                    save_grid(file, grid, it)
    else:
        
        with mp.Pool(processes=processes, initializer=init_worker, initargs=(grid,)) as pool:
            for _ in range(1, iterations + 1):
                grid = new_generation_parallel(grid, pool, processes)
                init_worker(grid)



if __name__ == "__main__":
    start = time.perf_counter()

    mp.freeze_support()
    run_simulation(rows=10, cols=10, iterations=10, processes=mp.cpu_count(), output_path="python/output/par_states.txt")

    end = time.perf_counter()

    print(f"Execution time: {end - start:.4f} seconds")