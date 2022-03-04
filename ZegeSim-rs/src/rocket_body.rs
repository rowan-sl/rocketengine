use crate::{control::TVC, math::Vec3, motor::RocketEngineSystem, physics::PhysicsBody};

/// thing that go up
pub struct RocketBody {
    body: PhysicsBody,

    time: f32,
    time_step: f32,

    //TODO add sensors herer
    tvc: TVC,
    tvc_position: Vec3,
    tvc_location: Vec3,
    reaction_wheel_torque: f32,

    //TODO make rocket engines (motors.py)
    rocket_motor: RocketEngineSystem,
    cp_location: Vec3,
    dry_mass: f32,
}

impl RocketBody {
    pub fn new() -> Self {
        Self {
            body: PhysicsBody::new(),

            time: 0.0,
            time_step: 0.0,

            tvc: TVC::new(),
            tvc_position: Vec3::default(),
            tvc_location: Vec3::default(),
            reaction_wheel_torque: 0.0,

            rocket_motor: RocketEngineSystem::new(1000.0, 0.0), //TODO what is these values mean
            cp_location: Vec3::default(),
            dry_mass: 1.0,
        }
    }

    pub fn get_time_seconds(&self) -> f32 {
        self.time
    }

    pub fn get_time_milliseconds(&self) -> f32 {
        self.time * 1000.0
    }

    pub fn get_time_microseconds(&self) -> f32 {
        self.time * 1000000.0
    }

    pub fn update(&mut self) {
        self.body.mass = self.dry_mass + self.rocket_motor.total_mass();

        self.rocket_motor.update(self.time);
        self.tvc.actuate(self.tvc_position, self.time_step);

        self.tvc
            .calculate_forces(self.rocket_motor.current_thrust());
        self.body.add_force_local(self.tvc.force);
        self.body.add_torque_local(
            Vec3::new(0.0, self.tvc.force.y, self.tvc.force.z) * self.tvc_location.x,
        );

        self.body.update_aero();
        self.body.add_force(self.body.drag_force);
        self.body
            .add_torque(Vec3::new(0.0, self.body.drag_force.y, self.body.drag_force.z) * -0.15);
        self.body.update(self.time_step);
    }

    pub fn clear(&mut self) {
        self.body.clear();
    }
}
