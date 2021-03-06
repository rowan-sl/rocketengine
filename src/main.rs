pub mod math;
pub mod motor;
pub mod consts;
pub mod rocket;
pub mod controll;
pub mod logger;
pub mod physics;

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
https://www.thrustcurve.org/
https://www.youtube.com/watch?v=nwgd1CV__rs&t=128s
*/

// ajust acordingly
const TICK_TIME: f64 = 0.1;
const SECS_TO_TICKS: usize = 10;
const SIM_STEPS: usize = 10 * SECS_TO_TICKS;

fn main() {
    let factory = RocketFactory::with_mass(800.0)
        .add_engine(
            "ascent".into(),
            Motor::new(
                motor::raw::E12::DATA.to_vec(),
                motor::raw::E12::DRY_WEIGHT,
            )
        );

    let mut r = factory.at(F64x3::new(0.0, 0.0, 5.0));

    let mut logger = Logger::open_file("out/launch.csv".into()).unwrap();

    r.light_engine("ascent".into());

    for t in 0..SIM_STEPS {
        r.tick(TICK_TIME);
        let time = t as f64 * TICK_TIME;
        r.log(time, &mut logger);
    }
}
