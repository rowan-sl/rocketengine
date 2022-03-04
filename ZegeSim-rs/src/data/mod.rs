pub mod flight_path;
pub mod logger;
pub mod settings_parser;

use crate::math::Vec3;

pub use flight_path::FlightPath;
pub use settings_parser::Settings;

/**
Class storing all the control outputs of the rocket

Params:

tvc_position - a vector represenging the position of the rocket's thrust vector control mount in radians

reaction_wheel_output - a torque in newton-meters representing the torque from a reaction wheel

motor_fire - the name of the motor you want to fire, leave blank to not fire any motor
*/
pub struct ControllData {
    pub tvc_position: Vec3,
    pub reaction_wheel_output: f32,
    pub motor_fire: String,
}
