use std::{fs::OpenOptions, path::PathBuf};

use csv::ReaderBuilder;

use crate::math::Vec3;

#[derive(Debug, Default, Clone)]
pub struct FlightPath {
    setpoint: [f32; 4],
    setpoints: Vec<[f32; 4]>,
    current_setpoint: Vec3,
}

impl FlightPath {
    pub fn from_file(path: PathBuf) -> Self {
        let mut reader = ReaderBuilder::new().from_path(path).unwrap();

        let mut setpoints = vec![];

        for row_raw in reader.records() {
            let row = row_raw.unwrap();
            setpoints.push([
                row[0].parse::<f32>().unwrap(),
                row[1].parse::<f32>().unwrap(),
                row[2].parse::<f32>().unwrap(),
                row[3].parse::<f32>().unwrap(),
            ]);
        }
        Self {
            setpoints,
            ..Default::default()
        }
    }

    pub fn get_next_setpoint(&mut self, time: f32) -> Vec3 {
        let mut setpoint_last: [f32; 4] = Default::default();
        let mut setpoint_future: [f32; 4] = Default::default();

        for (index, datapoint) in self.setpoints.iter().enumerate() {
            if datapoint[0] < time {
                setpoint_last = *datapoint;
            }

            if datapoint[0] > time && setpoint_future == <[f32; 4]>::default() {
                setpoint_future = *datapoint;
            }

            if index == self.setpoints.len() - 1 && setpoint_future == <[f32; 4]>::default() {
                setpoint_future = self.setpoints[self.setpoints.len() - 1];
            }
        }

        if setpoint_future != <[f32; 4]>::default() {
            let time_diff = setpoint_future[0] - setpoint_last[0];
            let setpoint_diff =
                Vec3::new(setpoint_future[1], setpoint_future[2], setpoint_future[3])
                    - Vec3::new(setpoint_last[1], setpoint_last[2], setpoint_last[3]);
            let rate_of_change = setpoint_diff / time_diff;
            self.current_setpoint = Vec3::new(setpoint_last[1], setpoint_last[2], setpoint_last[3])
                + rate_of_change * (time - setpoint_last[0]);
        }

        return self.current_setpoint;
    }
}
