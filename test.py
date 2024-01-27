import preprocess_cancellation
import inspect

file_path = './dragon.gcode'
tmp_file = './tmp.gcode'

with open(file_path, 'r') as in_file:
    with open(tmp_file, 'w') as out_file:
        for line in preprocess_cancellation.preprocess_slicer(in_file):
            out_file.write(line)
