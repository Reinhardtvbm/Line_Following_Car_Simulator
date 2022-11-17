import matplotlib.pyplot as plt
import numpy as np


def read_file(filename):
    file = open(filename)
    list = file.readlines()


    x_vals = []
    y_vals = []

    for line in list:
        split = line.split(',')
        split[1] = split[1].strip('\n')

        x_vals.append(float(split[0]))
        y_vals.append(float(split[1]))
    
    return [x_vals, y_vals]

axle = read_file('output.txt')
sensor0 = read_file('sensor0.txt')
sensor1 = read_file('sensor1.txt')
sensor2 = read_file('sensor2.txt')
sensor3 = read_file('sensor3.txt')
sensor4 = read_file('sensor4.txt')
sensor5 = read_file('sensor5.txt')
sensor6 = read_file('sensor6.txt')
pid = read_file('pid_out.txt')

sin_seq = []

for x in axle[0]:
    sin_seq.append(50*np.cos(x*0.05 - 1.0) - 50)

fig, axs = plt.subplots(2)

axs[0].plot(
axle[0], axle[1], 
# sensor0[0], sensor0[1], 
# sensor1[0], sensor1[1], 
# sensor2[0], sensor2[1], 
# sensor3[0], sensor3[1], 
# sensor4[0], sensor4[1], 
# sensor5[0], sensor5[1], 
# sensor6[0], sensor6[1],
axle[0], sin_seq)

axs[1].plot(pid[0], pid[1])
plt.show()
