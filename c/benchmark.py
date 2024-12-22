import time
import random

N = 1000000  # Number of elements for the benchmark

# Generate random integers
data = [str(random.randint(0, 1 << 30)) for _ in range(N)]

# Benchmark insertion
start = time.time()
py_set = set()
for value in data:
    py_set.add(value)
end = time.time()
print(f"Insertion Time: {end - start:.2f} seconds")

# Benchmark lookup
start = time.time()
for value in data:
    value in py_set
end = time.time()
print(f"Lookup Time: {end - start:.2f} seconds")

# Benchmark deletion
start = time.time()
for value in data:
    if value in py_set:
        py_set.remove(value)
end = time.time()
print(f"Deletion Time: {end - start:.2f} seconds")

