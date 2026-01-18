import random
from typing import List

Grid = List[List[int]]


def create_random_grid(rows: int, cols: int, alive_probability: float = 0.2):
    grid = []

    for _ in range(rows):
        row = []
        for _ in range(cols):
            if random.random() < alive_probability:
                row.append(1)
            else:
                row.append(0)

        grid.append(row)

    return grid
    

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

def count_alive_neighbors(grid: Grid, row: int, col: int):
    rows = len(grid)
    cols = len(grid[0])
    alive = 0

    for neighbor_rows in [-1, 0, 1]:
        for neighbor_cols in [-1, 0, 1]:
            if neighbor_rows == 0 and neighbor_cols == 0:
                continue

            r = row + neighbor_rows
            c = col + neighbor_cols

            if 0 <= r < rows and 0 <= c < cols:
                alive += grid[r][c]

    return alive


def save_grid(file, grid: Grid, iteration: int):
    file.write(f"Iteration {iteration}\n")
    for row in grid:
        for cell in row:
            file.write("".join(str(cell)))
        file.write("\n")
    file.write("\n")

def run_simulation(rows: int, cols: int, iterations: int, output_path: str):
 
    grid = create_random_grid(rows, cols)

    with open(output_path, "w") as file:
        save_grid(file, grid, 0)

        for it in range(1, iterations + 1):
            grid = new_generation(grid)
            save_grid(file, grid, it)


if __name__ == "__main__":
    run_simulation(rows=10, cols=10, iterations=100, output_path="python/sequential/output/states.txt")
