import time
import csv
import statistics
from python.parallel.game_of_life import run_simulation


ROWS = 1600
COLS = 1600
ITERATIONS = 100

REPEATS = 2
PROCESS_COUNTS = [1, 2, 4, 8, 16] 

OUTPUT_CSV = "python/output/strong_scaling_results.csv"


def measure_execution_time(processes: int) -> float:
    start = time.perf_counter()
    run_simulation(
        rows=ROWS,
        cols=COLS,
        iterations=ITERATIONS,
        processes=processes,
        output_path=None
    )
    end = time.perf_counter()
    return end - start


def main():
    results = []

    for p in PROCESS_COUNTS:
        times = []
        print(f"Running strong scaling for {p} processes...")

        for i in range(REPEATS):
            t = measure_execution_time(p)
            times.append(t)
            print(f"  Run {i + 1}/{REPEATS}: {t:.4f}s")

        avg = statistics.mean(times)
        std = statistics.stdev(times)

        results.append((p, avg, std))

    with open(OUTPUT_CSV, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["Processes", "AvgTime(s)", "StdDev(s)"])
        writer.writerows(results)


if __name__ == "__main__":
    main()
