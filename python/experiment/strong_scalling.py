import time
import csv
import statistics

from python.parallel.game_of_life import run_simulation as run_parallel
from python.sequential.game_of_life import run_simulation as run_sequential  


ROWS = 10
COLS = 10
ITERATIONS = 100

REPEATS = 1
PROCESS_COUNTS = [1, 2, 4, 8, 16]

OUTPUT_CSV = "python/output/strong_scaling_results.csv"


def measure_execution_time_parallel(processes: int) -> float:
    start = time.perf_counter()
    run_parallel(
        rows=ROWS,
        cols=COLS,
        iterations=ITERATIONS,
        processes=processes,
        output_path=None
    )
    end = time.perf_counter()
    return end - start


def measure_execution_time_sequential() -> float:
    start = time.perf_counter()
    run_sequential(
        rows=ROWS,
        cols=COLS,
        iterations=ITERATIONS,
        output_path=None
    )
    end = time.perf_counter()
    return end - start


def safe_std(times):
   
    return statistics.stdev(times) if len(times) >= 2 else 0.0


def main():
    results = []

    seq_times = []
    print("Running strong scaling sequential...")
    for i in range(REPEATS):
        t = measure_execution_time_sequential()
        seq_times.append(t)
        print(f"  Run {i + 1}/{REPEATS}: {t:.4f}s")

    seq_avg = statistics.mean(seq_times)
    seq_std = safe_std(seq_times)
    results.append(("seq", seq_avg, seq_std))

    for p in PROCESS_COUNTS:
        times = []
        print(f"Running strong scaling for {p} processes...")

        for i in range(REPEATS):
            t = measure_execution_time_parallel(p)
            times.append(t)
            print(f"  Run {i + 1}/{REPEATS}: {t:.4f}s")

        avg = statistics.mean(times)
        std = safe_std(times)

        results.append((p, avg, std))

    with open(OUTPUT_CSV, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["Processes", "AvgTime(s)", "StdDev(s)"])
        writer.writerows(results)

    print(f"Wrote {OUTPUT_CSV}")


if __name__ == "__main__":
    main()