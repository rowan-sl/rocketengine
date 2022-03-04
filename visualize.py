# import matplotlib.pyplot as plt
# import pandas as pd

from vis.plotter import plotter;

# df = pd.read_csv("launch.csv")

# plt.plot(df['time'], df['location'])
# plt.plot(df['time'], df['velocity'])
# plt.plot(df['time'], df['acceleration'])

# plt.show()

p = plotter()
p.read_header("launch.csv")

# p.create_2d_graph(['time', 'vel_x', 'vel_y', 'vel_z'], 'time (sec)', 'velocity (m/s)', True)
# p.create_2d_graph(['time', 'accel_x', 'accel_y', 'accel_z'], 'time (sec)', 'acceleration (m/s^2)', True)
# p.create_2d_graph(['time', 'mass'], 'time (sec)', 'weight', True)

p.create_2d_graph(['time', 'accel_x', 'accel_y', 'accel_z', 'accel_ng_x', 'accel_ng_y', 'accel_ng_z', 'vel_x', 'vel_y', 'vel_z'], 'time (sec)', 'accel (m/s^2), accel (no grav), velocity (m/s)', True)
p.create_2d_graph(['time', 'mass'], 'time (sec)', 'weight (g)', True)

p.create_3d_graph(['pos_x', 'pos_y', 'pos_z'], 70.0)

p.create_3d_animation(['time', 'pos_x', 'pos_y', 'pos_z'], 70.0, 0.1)

p.show_all_graphs()

