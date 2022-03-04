pub fn interpolate_thrust(thrust_curve: &Vec<[f32; 2]>, time_step: f32) -> Vec<f32> {
    let mut thrust_list = vec![];
    let mut l_point: [f32; 2] = [0.0, 0.0];
    for point in thrust_curve {
        if point[0] > 0.0 {
            let thrust_diff = point[1] - l_point[1];
            let time_diff = point[0] - l_point[0];
            let steps_needed = time_diff * time_step;

            if steps_needed > 0.0 {
                let adder = thrust_diff / steps_needed;

                let mut i = 0.0;

                let mut thrust_to_add = l_point[1];

                while i < steps_needed {
                    i += 1.0;
                    thrust_to_add += adder;
                    thrust_list.push(thrust_to_add);
                }
            }
        }
        l_point = *point;
    }
    thrust_list
}
