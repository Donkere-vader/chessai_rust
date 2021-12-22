import json
from matplotlib import pyplot as plt
from rich import print
import numpy as np

with open("benchmarks.json", "r") as f:
    data = json.load(f)

BAR_WIDTH = 1.0 / (len(data.keys()) + 2)
ind = np.arange(len(data.keys()))

groups = []
fen_code_times = {}

for group in data:
    groups.append(group)
    for fen_code in data[group]['results']:
        if fen_code not in fen_code_times:
            fen_code_times[fen_code] = []
        fen_code_times[fen_code].append(data[group]['results'][fen_code]['time'])


for (i, fen_code) in enumerate(fen_code_times):
    plt.bar(ind + i * BAR_WIDTH, fen_code_times[fen_code], BAR_WIDTH, label=fen_code)



plt.title("Benchmark data")
plt.xlabel('FEN Code', fontweight ='bold', fontsize = 15)
plt.ylabel('Microseconds', fontweight ='bold', fontsize = 15)
plt.xticks(ind + BAR_WIDTH + (BAR_WIDTH / 2), groups)
 
plt.legend(loc='best')
plt.show()
