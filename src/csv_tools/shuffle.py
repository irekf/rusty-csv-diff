import linecache
import random

def shuffle_csv(input_path, output_path):
    row_num = 0
    with open(input_path, 'r') as f:
        for row_num, _ in enumerate(f):
            pass

    input_index = [i for i in range(row_num)]
    output_index = random.sample(input_index[1:], len(input_index) - 1)
    output_index = [0] + output_index

    with open(output_path, "w+") as fout:
        for i in output_index:
            row = linecache.getline(input_path, i+1)
            fout.write(row)
