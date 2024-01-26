import preprocess_cancellation
import inspect

print(inspect.getmembers(preprocess_cancellation, lambda x: True))
preprocess_cancellation.preprocess_slicer('./superslicer.gcode')
