use crate::math::{clamp, Vec3, DEG_TO_RAD, RAD_TO_DEG};

#[derive(Debug, Default, Clone)]
pub struct TVC {
    command: Vec3,

    position: Vec3,

    servo_position: Vec3,

    min: Vec3,
    max: Vec3,

    offset: Vec3,

    // FIXME unused?
    // noise: Vec3,
    servo_speed: f32,

    linkage_ratio: f32,

    pub force: Vec3,
}

impl TVC {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actuate(&mut self, command_angles: Vec3, dt: f32) {
        self.command.y = command_angles.y * RAD_TO_DEG * self.linkage_ratio;
        self.command.z = command_angles.z * RAD_TO_DEG * self.linkage_ratio;

        let actuation_y = clamp(
            self.command.y - self.servo_position.y,
            -self.servo_speed,
            self.servo_speed,
        );
        let actuation_z = clamp(
            self.command.z - self.servo_position.z,
            -self.servo_speed,
            self.servo_speed,
        );

        self.servo_position.y += actuation_y * dt;
        self.servo_position.z += actuation_z * dt;

        self.servo_position.y = clamp(self.servo_position.y, self.min.y, self.max.y);
        self.servo_position.z = clamp(self.servo_position.z, self.min.z, self.max.z);

        self.position.y =
            ((self.servo_position.y + self.offset.y) / self.linkage_ratio) * DEG_TO_RAD;
        self.position.z =
            ((self.servo_position.z + self.offset.z) / self.linkage_ratio) * DEG_TO_RAD;
    }

    pub fn calculate_forces(&mut self, thrust: f32) {
        self.force.y = (self.position.y).sin() * thrust;
        self.force.z = (self.position.z).sin() * thrust;
        self.force.x =
            thrust * (self.position.y).cos() - (thrust - (thrust * (self.position.z).cos()));
    }
}
