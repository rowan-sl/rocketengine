use crate::math::{Quaternion, Vec3};

//TODO use builder pattern instead of this public field bs
/**
Class representing a rigid body in 3 dimensional space.

    parameters:

    mass : f32
        mass of the body

    position : Vec3
        position of the body

    velocity : Vec3
        velocity of the body

    angular_velocity : Vec3
        angular velocity of the body

    orientation : quaternion
        orientation of the body as a quaternion

    moment_of_inertia : Vec3
        moment of inertia of the body
*/
pub struct PhysicsBody {
    pub position: Vec3,
    pub velocity: Vec3,

    pub acceleration: Vec3,
    pub acceleration_local: Vec3,

    pub gravity: Vec3,

    pub rotation: Quaternion,
    pub rotation_euler: Vec3,
    pub rotational_velocity: Vec3,

    pub rotational_velocity_local: Vec3,

    pub rotational_acceleration: Vec3,

    pub mass: f32,
    pub moment_of_inertia: Vec3,

    pub floor: bool,

    pub wind: Vec3,
    pub drag_force: Vec3,
    pub drag_area: f32,
    pub drag_coefficient: f32,

    pub aoa: f32,
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            velocity: Vec3::default(),

            acceleration: Vec3::default(),
            acceleration_local: Vec3::default(),
            gravity: Vec3::default(),

            rotation: Quaternion::default(),
            rotation_euler: Vec3::default(),
            rotational_velocity: Vec3::default(),

            rotational_velocity_local: Vec3::default(),

            rotational_acceleration: Vec3::default(),

            mass: 1.0,
            moment_of_inertia: Vec3::default(),

            floor: true,

            wind: Vec3::default(),
            drag_force: Vec3::default(),
            drag_area: 0.0,
            drag_coefficient: 0.0,
            aoa: 0.0,
        }
    }
}

impl PhysicsBody {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a force to the body
    pub fn add_force(&mut self, force: Vec3) {
        self.acceleration += force / self.mass;
    }

    /// Add a torque to the body.
    pub fn add_torque(&mut self, torque: Vec3) {
        self.rotational_acceleration += torque / self.moment_of_inertia;
    }

    /// Add a force to the body in local space.
    pub fn add_force_local(&mut self, force: Vec3) {
        let nf = self.rotation.rotate(force);

        self.add_force(nf);
    }

    /// Add a torque to the body in local space.
    pub fn add_torque_local(&mut self, torque: Vec3) {
        let nt = self.rotation.rotate(torque);

        self.add_torque(nt);
    }

    /// Add a force to the body at a point in global space.
    pub fn add_global_point_force(&mut self, force: Vec3, point: Vec3) {
        let mut torque = force * point.x;
        torque.x = 0.0;
        self.add_torque(torque);

        self.add_force(force);
    }

    /// Add a force to the body at a point.
    pub fn add_local_point_force(&mut self, force: Vec3, point: Vec3) {
        let nf = self.rotation.rotate(force);
        // unused?
        // let np = self.rotation.rotate(point);

        self.add_global_point_force(nf, point);
    }

    /// Updates aerodynamic forces acting on the body.
    /// Note - you still need to apply the drag force to the physics body with apply_glocal_point_force().
    pub fn update_aero(&mut self) {
        let velocity_relative_wind = self.velocity - self.wind;

        if velocity_relative_wind.x != 0.0
            && velocity_relative_wind.y != 0.0
            && velocity_relative_wind.z != 0.0
        {
            self.aoa = velocity_relative_wind
                .angle_between(self.rotation.rotate(Vec3::new(1.0, 0.0, 0.0)));

            let dc = self.drag_coefficient * self.aoa;

            if self.floor == false && self.position.x != 0.0 {
                self.drag_force = -velocity_relative_wind.normalize()
                    * 0.5
                    * 1.225
                    * (self.velocity.norm().powi(2))
                    * dc
                    * self.drag_area;
            } else if self.floor == false {
                self.drag_force = -velocity_relative_wind.normalize()
                    * 0.5
                    * 1.225
                    * (self.velocity.norm().powi(2))
                    * dc
                    * self.drag_area;
            }
        }
    }
    /// Updates the physics body
    pub fn update(&mut self, dt: f32) {
        self.acceleration_local = self.rotation.conj().rotate(self.acceleration);

        self.acceleration += self.gravity;

        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;

        self.rotational_velocity += self.rotational_acceleration * dt;

        let mut ang = self.rotational_velocity.norm();

        if ang == 0.0 {
            //lol
            ang = 0.000000001;
        }

        self.rotation *= Quaternion::from_axis_angle(self.rotational_velocity / ang, ang * dt);

        self.rotation_euler = self.rotation.to_euler();

        self.rotational_velocity_local = self.rotation.conj().rotate(self.rotational_velocity);

        // self.rotation_euler.x = 0.0
        // self.rotational_velocity.x = 0.0
        // self.rotation = quaternion().euler_to_quaternion(self.rotation_euler)

        if self.floor && self.position.x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x = 0.0;
        }
    }

    /// clears the rotational&&translational acceleration
    pub fn clear(&mut self) {
        self.acceleration = Vec3::new(0.0, 0.0, 0.0);
        self.rotational_acceleration = Vec3::new(0.0, 0.0, 0.0);
    }
}
