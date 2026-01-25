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


def count_alive_neighbors(grid: Grid, row: int, col: int):
    rows = len(grid)
    cols = len(grid[0])
    alive = 0

    for neighbor_rows in (-1, 0, 1):
        for neighbor_cols in (-1, 0, 1):
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
