import json
import chess_ai
from datetime import datetime as dt
from rich import print
import os


class BenchMarker():
    def __init__(self, bench_name, runs):
        self.bench_name = bench_name
        with open("fencodes.json", 'r') as f:
            self.fen_codes = json.load(f)
        self.runs = runs
        
    
    def run(self):
        bench_data = {}
        bench_data['results'] = {}

        for idx, key in enumerate(self.fen_codes):
            times = []
            print(f"[BENCHMARK {idx + 1}/{len(self.fen_codes)}]", key.ljust(30), end=" ")
            for i in range(self.runs):
                start = dt.now()
                best_move = chess_ai.get_best_move(self.fen_codes[key], 4, False)
                elapsed = dt.now() - start
                elapsed_micro = elapsed.microseconds + elapsed.seconds * 1_000_000
                times.append(elapsed_micro)
                print(i + 1, end=" ")
            print()
            bench_data['runs'] = self.runs
            bench_data['results'][key] = sum(times) / len(times)

        print()
        print("[ === DATA === ]")
        for key in bench_data['results']:
            print(key.ljust(20), bench_data['results'].get(key))
        
        if os.path.exists("benchmarks.json"):
            with open("benchmarks.json", "r") as f:
                data_in_file = json.load(f)
        else:
            data_in_file = {}
        
        data_in_file[self.bench_name] = bench_data

        with open("benchmarks.json", "w") as f:
            json.dump(data_in_file, f, indent=4)
