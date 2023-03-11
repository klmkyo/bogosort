from copy import deepcopy
from datetime import datetime
import subprocess
import time
import pathlib
import sys

# function that creates a markdown table with the results of the benchmark
# it takes an array of dictionaries with {header: value}
# and returns a string with the markdown table
def create_table(results: list[dict[str, float]]) -> str:
    # get the headers from the first result
    headers = results[0].keys()
    # create the header row
    header_row = "| " + " | ".join(headers) + " |"
    # create the separator row
    separator_row = "| " + " | ".join(["---"] * len(headers)) + " |"
    # create the rows with the results
    rows = []
    for result in results:
        row = "| " + " | ".join([str(result[header]) for header in headers]) + " |"
        rows.append(row)
    # join the rows together
    table = "\n".join([header_row, separator_row, *rows])
    return table

# from an array of execution times in µs, return the average, min and max, standard deviation and median
def get_stats(times: list[float]) -> dict[str, float]:
    times = deepcopy(times)
    
    # if results len > 3, remove the min and max
    sorted_times = sorted(times)
    if len(times) > 3:
        times = sorted_times[1:-1]
        sorted_times = sorted(times)
    
    average = sum(times) / len(times)
    min_time = min(times)
    max_time = max(times)
    std_dev = (sum([(time - average) ** 2 for time in times]) / len(times)) ** 0.5
    median = sorted_times[len(times) // 2]
    return {
        "average": average,
        "min": min_time,
        "max": max_time,
        "std_dev": std_dev,
        "median": median
    }

# given time in µs, return either µs, ms or s depending on the time
# optionally accept second argument for standard deviation (time<unit> ± std_dev)
# does not display trailing zeros
def format_time(time_in_microseconds, std_dev=None):
    units = ['µs', 'ms', 's']
    thresholds = [1000, 1000, float('inf')]
    val = time_in_microseconds
    
    unit = units[0]
    for i, thresh in enumerate(thresholds):
        if val < thresh:
            break
        val /= thresh
        if std_dev is not None:
            std_dev /= thresh
        unit = units[i+1]
        
    # remove trailing zeros
    val = f"{val:.2f}".rstrip("0").rstrip(".")
    if std_dev is not None:
        std_dev = f"{std_dev:.2f}".rstrip("0").rstrip(".")
        
    if std_dev is None:
        return f"{val} {unit}"
    else:
        return f"{val} {unit} ± {std_dev}"


    
def measure_execution_time(program: str, args: list[str]) -> float:
    result = subprocess.Popen((PROGRAM, *args), stdout=subprocess.PIPE)
    
    # wait for the program to finish
    result.wait()
    
    # get the execution time from the program
    output = result.stdout.read().decode("utf-8").strip()
    execution_time_micro = float(output)
    return execution_time_micro


PROGRAM = "./target/release/bogosort"

# wait for the compilation to finish

# Define the arguments to pass to the program
ARGS = [ [x] for x in range(1, 14)]

# Loop over the arguments and run the program

all_results = {}

# for ctrl c
wants_to_quit = False

for arggarr in ARGS:
    
    # make sure the arguments are strings
    arggarr = [str(arg) for arg in arggarr]
    
    # measure the execution time
    # unpack the arguments
    # stdout = subprocess.PIPE
    results = []
    
    # if the user ctrl+c's the program, print the results so far
    
    try:
        # warmup runs
            for _ in range(2):
                print(f"Warmup: n={arggarr[0]}", end=" ")
                sys.stdout.flush()
                execution_time_micro = measure_execution_time(PROGRAM, arggarr)
                print(f"{format_time(execution_time_micro)}")

            for _ in range(30):
                print(f"n={arggarr[0]}", end=" ")
                sys.stdout.flush()
                execution_time_micro = measure_execution_time(PROGRAM, arggarr)
                results.append(execution_time_micro)
                print(f"{format_time(execution_time_micro)}")
                # print(f"n={arggarr[0]}: {format_time(execution_time_micro)}")
            
    except KeyboardInterrupt:
        wants_to_quit = True
        print("KeyboardInterrupt")       

    all_results[arggarr[0]] = results
    
    if wants_to_quit:
        break

# save all_results to json file
import json
json_path = "benchmarks"
# with current date and time
filename = "results_" + datetime.now().strftime("%Y-%m-%d_%H-%M-%S") + ".json"
full_path = pathlib.Path(json_path, filename)

with open(full_path, "w") as f:
    json.dump(all_results, f)


table = []

for result in all_results:
    k, v = result, all_results[result]
    
    # check for empty results
    if len(v) == 0:
        continue
    
    stats = get_stats(v)
    
    table.append({
        "n": k,
        "Czas średni": format_time(stats['average'], stats['std_dev']),
        "Czas min.": format_time(stats['min']),
        "Czas max.": format_time(stats['max'])
    })

# print the results in a markdown table
print(create_table(table))