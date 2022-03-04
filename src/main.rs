pub mod math;
pub mod motor;
pub mod consts;
pub mod rocket;
mod logger;
pub use consts::*;
use logger::Logger;
use math::F64x3;
use motor::Motor;
use rocket::RocketFactory;

/*
x and y are the two horizontal axis, and z is the vertical axis

time is in seconds (so m/s works out)

velocity = change in location / change in time
acceleration = change in velocity / time

force = mass * acceleration
force / mass = acceleration (!!!!!)

TODO https://en.wikipedia.org/wiki/Newton%27s_laws_of_motion#Variable-mass_systems
*/

// ajust acordingly
const TICK_TIME: f64 = 0.1;
const SECS_TO_TICKS: usize = 10;
const SIM_STEPS: usize = 10 * SECS_TO_TICKS;

fn main() {
    let factory = RocketFactory::with_mass(500.0)
        .add_engine("ascent".into(), Motor::new(
            motor::spec::MotorSpec::C6.into_raw_thrust(),
            0.05,// half of time step
            motor::spec::c6::DRY_WEIGHT,
            motor::spec::c6::WET_WEIGHT,
        ));

    let mut r = factory.at(F64x3::new(0.0, 0.0, 0.0));

    let mut logger = Logger::open_file("launch.csv".into()).unwrap();

    r.light_engine("ascent".into());

    for t in 0..SIM_STEPS {
        r.tick(TICK_TIME);
        let time = t as f64 * TICK_TIME;
        r.log(time, &mut logger);
    }
}
