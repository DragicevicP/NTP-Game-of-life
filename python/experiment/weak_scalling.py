import time
import csv
import statistics
from python.parallel.game_of_life import run_simulation


BASE_SIZE = 100        
ITERATIONS = 100
REPEATS = 3

PROCESS_COUNTS = [1, 2, 4, 8, 16]

OUTPUT_CSV = "python/output/weak_scaling_results.csv"


def grid_size_for_processes(p: int):
    return BASE_SIZE * p, BASE_SIZE


def measure_execution_time(processes: int) -> float:
    rows, cols = grid_size_for_processes(processes)

    start = time.perf_counter()
    run_simulation(
        rows=rows,
        cols=cols,
        iterations=ITERATIONS,
        processes=processes,
        output_path=None
    )
    end = time.perf_counter()

    return end - start


def main():
    results = []

    for p in PROCESS_COUNTS:
        rows, cols = grid_size_for_processes(p)
        times = []

        print(f"Running weak scaling for {p} processes")

        for i in range(REPEATS):
            t = measure_execution_time(p)
            times.append(t)
            print(f"  Run {i + 1}/{REPEATS}: {t:.4f}s")

        avg = statistics.mean(times)
        std = statistics.stdev(times)

        results.append((p, rows, cols, avg, std))

    with open(OUTPUT_CSV, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow([
            "Processes",
            "Rows",
            "Cols",
            "AvgTime(s)",
            "StdDev(s)"
        ])
        writer.writerows(results)


if __name__ == "__main__":
    main()
