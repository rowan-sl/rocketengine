use std::collections::HashMap;

use crate::math::F64x3;
use crate::motor::Motor;
use crate::logger::{CSVRow, Logger};
use crate::consts::*;

#[derive(Debug)]
pub struct Rocket {
    /// meters
    location: F64x3,
    /// figure out
    velocity: F64x3,
    /// m/s^2
    acceleration: F64x3,
    /// kg
    dry_mass: f64,
    /// kg (calculated with recalc_mass, not set manualy)
    mass: f64,
    /// constant forces acting on the rocket, used as force / mass = acceleration to calculate acceleration
    /// 
    /// measured in newtons (kg*m)/second^2
    const_forces: Vec<F64x3>,
    /// gravity. applied as an acceleration
    /// 
    /// TODO make this use a approximation with distance from sea level instead of just a constant acceleration
    gravity: F64x3,
    /// Rocket Engines
    engines: HashMap<String, Motor>,
}

impl Rocket {
    pub fn recalc_mass(&mut self) {
        let mut mass = self.dry_mass;
        for engine in self.engines.values() {
            mass += engine.weight();
        }
        self.mass = mass;
    }

    pub fn add_engine(&mut self, name: String, engine: Motor) {
        match self.engines.insert(name.clone(), engine) {
            Some(_) => {panic!("Engine {} already exists!", name)},
            None => {},
        }
        self.recalc_mass();
    }

    pub fn light_engine(&mut self, name: String) {
        if let Some(engine) = self.engines.get_mut(&name) {
            if !engine.lit() {
                engine.light()
            }
        }
    }

    pub fn log(&mut self, time: f64, logger: &mut Logger) {
        logger.write_record(CSVRow {
            time,
            pos_x: self.location.z,
            pos_y: self.location.x,
            pos_z: self.location.y,
            accel_x: self.acceleration.x,
            accel_y: self.acceleration.y,
            accel_z: self.acceleration.z,
            accel_ng_x: self.acceleration.x - GRAVITY.x,
            accel_ng_y: self.acceleration.y - GRAVITY.y,
            accel_ng_z: self.acceleration.z - GRAVITY.z,
            vel_x: self.velocity.x,
            vel_y: self.velocity.y,
            vel_z: self.velocity.z,
            mass: self.mass * KG_TO_GRAMS,
        }).unwrap();
    }
}

impl Rocket {
    fn calc_engine_thrusts(&mut self, dt: secs) -> F64x3 {
        let mut total = F64x3::zero();

        for engine in self.engines.values_mut() {
            total += engine.thrust_for(dt).unwrap_or(F64x3::zero());
        }

        total
    }

    fn calc_accel(&mut self, dt: secs) {
        // acceleration = force / mass, so sum up all constant forces, and then devide them by the mass of the rocket
        let mut total_force: F64x3 = F64x3::zero();

        self.const_forces.iter().for_each(|i| {total_force += *i});

        let engine_thrust = self.calc_engine_thrusts(dt);
        total_force += engine_thrust;

        self.recalc_mass();

        self.acceleration = (total_force / self.mass) + self.gravity;
    }

    fn calc_velocity(&mut self, dt: secs) {
        // acceleration is velocity/time, so diff in velocity for a given period of time is acceleration * time
        let diff_in_velocity = self.acceleration * dt; 
        // add this new velocity to the rocket
        self.velocity += diff_in_velocity;
    }

    fn calc_location(&mut self, dt: secs) {
        // velocity is change in location / change in time, so velocity * time = change in location
        let change_in_location = self.velocity * dt;
        // move the rocket accordingly
        self.location += change_in_location;
    }

    pub fn tick(&mut self, dt: secs) {
        self.calc_accel(dt);
        self.calc_velocity(dt);
        self.calc_location(dt);
    }
}

pub struct RocketFactory {
    /// rocket mass in grams
    mass: f64,
    engines: HashMap<String, Motor>,
    const_forces: Vec<F64x3>,
}

impl RocketFactory {
    /// constructs a new rocket at the given location
    pub fn at(&self, location: F64x3) -> Rocket {
        let mut r = Rocket {
            location,
            velocity: F64x3::zero(),
            acceleration: F64x3::zero(),
            dry_mass: self.mass * GRAMS_TO_KG, 
            mass: 0.0,
            const_forces: self.const_forces.clone(),
            gravity: GRAVITY,
            engines: self.engines.clone(),
        };
        r.recalc_mass();
        r
    }

    /// Creates a new rocket factory, with the mass in grams
    pub fn with_mass(mass: f64) -> Self {
        Self {
            mass,
            engines: HashMap::new(),
            const_forces: vec![],
        }
    }

    pub fn new_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn add_engine(mut self, name: String, engine: Motor) -> Self {
        self.engines.insert(name, engine);
        self
    }

    pub fn add_const_force(mut self, force: F64x3) -> Self {
        self.const_forces.push(force);
        self
    }
}
