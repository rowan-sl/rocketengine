# import matplotlib.pyplot as plt
# import pandas as pd

from vis.plotter import plotter;

# df = pd.read_csv("launch.csv")

# plt.plot(df['time'], df['location'])
# plt.plot(df['time'], df['velocity'])
# plt.plot(df['time'], df['acceleration'])

# plt.show()

p = plotter("out/launch.csv")
p.read_header("out/launch.csv")

# p.create_2d_graph(['time', 'vel_x', 'vel_y', 'vel_z'], 'time (sec)', 'velocity (m/s)', True)
# p.create_2d_graph(['time', 'accel_x', 'accel_y', 'accel_z'], 'time (sec)', 'acceleration (m/s^2)', True)
# p.create_2d_graph(['time', 'mass'], 'time (sec)', 'weight', True)

p.create_2d_graph(['time', 'accel_x', 'accel_y', 'accel_z', 'vel_x', 'vel_y', 'vel_z'], 'time (sec)', 'accel (m/s^2), velocity (m/s)', True)
p.create_2d_graph(['time', 'mass'], 'time (sec)', 'weight (g)', True)

p.create_2d_graph(['time', 'pos_x', 'pos_y', 'pos_z'], 'time (secs)', 'location (meters)', True)

p.create_3d_graph(['pos_x', 'pos_y', 'pos_z'], 70.0)

p.create_3d_animation(['time', 'pos_x', 'pos_y', 'pos_z'], 70.0, 0.1)

p.show_all_graphs()

