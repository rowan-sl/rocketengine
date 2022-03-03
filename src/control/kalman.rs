// // TODO fix dis, as in the words of ZegesMended: its not how you do a kalman
// pub struct Kalman {
//     p: [f32; 4],
//     q: f32,
//     r: f32,
//     vel: f32,
//     pos: f32,

// }

// impl Kalman {
//     pub fn new() -> Self {
//         Self {
//             p: [0.0, 0.0, 0.0, 0.0],
//             vel: 0.0,
//             pos: 0.0,
//             q: 0.0,
//             r: 0.0,
//         }
//     }

//     pub fn update_accel(self, accel, dt) -> None:

//         self.pos += self.vel * dt + 0.5 * accel * (dt * dt)
//         self.vel += accel * dt
        
//         q_dtdt = self.Q * (dt * dt)
//         self.P[0] += (self.P[2] + self.P[1] +
//                     (self.P[3] + q_dtdt * 0.25) * dt) * dt
//         self.P[1] += (self.P[3] + q_dtdt * 0.5) * dt
//         self.P[2] = self.P[1]
//         self.P[3] += q_dtdt

//     pub fn update_position(self, pos) -> None:

//         y = self.pos - pos

//         inv = 1.0 / (self.P[0] + self.R)

//         K_a = self.P[0] * inv
//         K_b = self.P[2] * inv

//         self.pos += K_a * y
//         self.vel += K_b * self.vel

//         self.vel += y*10
//         self.vel /= 11.0

//         self.pos += pos
//         self.pos /= 2.0

//         self.P[0] -= K_a * self.P[0]
//         self.P[1] -= K_a * self.P[1]
//         self.P[2] -= K_b * self.P[0]
//         self.P[3] -= K_b * self.P[1]

//     pub fn get_position(self) -> float:

//         return self.pos

//     pub fn get_velocity(self) -> float:

//         return self.vel
// }