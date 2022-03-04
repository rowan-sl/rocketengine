use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use yaml_rust::YamlLoader;

use crate::math::Vec3;
use crate::motor::spec::MotorSpec;

#[derive(Clone, Debug)]
pub struct Settings {
    pub motors: Vec<(String, MotorSpec)>,
    pub max_ignition_delay: f32,

    pub time_step: f32,
    pub simulation_time: f32,

    pub imu_gyro_read_speed: f32,
    pub imu_accel_read_speed: f32,
    pub gps_read_speed: f32,
    pub baro_read_speed: f32,

    pub mass: f32,
    pub mmoi: Vec3,

    pub drag_area: f32,
    pub drag_coeff: f32,

    pub wind_speed: Vec3,

    pub tvc_noise: f32,

    pub tvc_servo_speed: f32,
    pub linkage_ratio: f32,

    pub max_tvc: Vec3,

    pub tvc_location: Vec3,
    pub cp_location: Vec3,
}

impl Settings {
    pub fn load(path: PathBuf) -> Self {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let config = YamlLoader::load_from_str(&buf).unwrap().remove(0)["settings"].to_owned();

        let mut motors = vec![];
        for point in config["motors"].to_owned().into_hash().unwrap() {
            motors.push((
                point.0.as_str().unwrap().to_string(),
                MotorSpec::parse_from_str(point.0.as_str().unwrap()).unwrap(),
            ));
        }

        let time_step = config["timeStep"].as_f64().unwrap() as f32;
        let simulation_time = config["simTime"].as_f64().unwrap() as f32;

        let imu_gyro_read_speed = config["gyroSpeed"].as_f64().unwrap() as f32;
        let imu_accel_read_speed = config["accelSpeed"].as_f64().unwrap() as f32;
        let gps_read_speed = config["gpsSpeed"].as_f64().unwrap() as f32;
        let baro_read_speed = config["baroSpeed"].as_f64().unwrap() as f32;

        let max_ignition_delay = config["max_motor_ignition_delay"].as_f64().unwrap() as f32;

        let mass = config["rocket_mass"].as_f64().unwrap() as f32;
        let drag_area = config["drag_area"].as_f64().unwrap() as f32;
        let drag_coeff = config["drag_coeff"].as_f64().unwrap() as f32;
        let tvc_noise = config["tvc_noise"].as_f64().unwrap() as f32;
        let tvc_servo_speed = config["tvc_servo_speed"].as_f64().unwrap() as f32;
        let linkage_ratio = config["tvc_linkage_ratio"].as_f64().unwrap() as f32;

        let max_tvc_raw = config["max_tvc_angle"].as_vec().unwrap().to_owned();
        let max_tvc = Vec3::new(
            0.0,
            max_tvc_raw[0].as_f64().unwrap() as f32,
            max_tvc_raw[1].as_f64().unwrap() as f32,
        );

        let wind_speed_raw = config["wind_speed"].as_vec().unwrap().to_owned();
        let wind_speed = Vec3::new(
            wind_speed_raw[0].as_f64().unwrap() as f32,
            wind_speed_raw[1].as_f64().unwrap() as f32,
            wind_speed_raw[2].as_f64().unwrap() as f32,
        );

        let mmoi_raw = config["mmoi"].as_vec().unwrap().to_owned();
        let mmoi = Vec3::new(
            mmoi_raw[0].as_f64().unwrap() as f32,
            mmoi_raw[1].as_f64().unwrap() as f32,
            mmoi_raw[2].as_f64().unwrap() as f32,
        );

        let tvc_location_raw = config["mmoi"].as_vec().unwrap().to_owned();
        let tvc_location = Vec3::new(
            tvc_location_raw[0].as_f64().unwrap() as f32,
            tvc_location_raw[1].as_f64().unwrap() as f32,
            tvc_location_raw[2].as_f64().unwrap() as f32,
        );

        let cp_location_raw = config["mmoi"].as_vec().unwrap().to_owned();
        let cp_location = Vec3::new(
            cp_location_raw[0].as_f64().unwrap() as f32,
            cp_location_raw[1].as_f64().unwrap() as f32,
            cp_location_raw[2].as_f64().unwrap() as f32,
        );

        Self {
            motors,
            max_ignition_delay,
            time_step,
            simulation_time,
            imu_accel_read_speed,
            imu_gyro_read_speed,
            gps_read_speed,
            baro_read_speed,
            mass,
            drag_area,
            drag_coeff,
            tvc_location,
            tvc_noise,
            tvc_servo_speed,
            linkage_ratio,
            max_tvc,
            wind_speed,
            cp_location,
            mmoi,
        }
    }
}
