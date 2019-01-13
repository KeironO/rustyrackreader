import matplotlib.pyplot as plt
import numpy as np

with open("/tmp/test", "r") as infile:
    data = infile.read().replace("[", "").replace("]", "")

data = np.array([float(x) for x in data.split(",")])

data = np.reshape(data, (480, 640))

plt.figure()
plt.imshow(data, cmap="gray")
plt.show()