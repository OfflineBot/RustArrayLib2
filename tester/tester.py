import numpy as np

x = np.array([
    [1.0, 2.0],
    [3.0, 4.0],
    [5.0, 6.0],
])

y = np.array([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
])
z = np.array([
    1.0, 2.0, 3.0
])

print(np.dot(x, y) + z)