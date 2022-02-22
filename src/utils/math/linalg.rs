use crate::utils::{math::trig, types};

pub fn mat6_mul3(m1: &[types::V6], m2: &[types::V3; 3]) -> Vec<types::V6> {
    let mut result = Vec::<types::V6>::new();
    for i in 0..m1.len() {
        let vertex = m1[i];
        let mut temp_result: types::V6 = [0.0, 0.0, 0.0, vertex[3], vertex[4], vertex[5]];
        for j in 0..3 {
            let mut sum = 0.0;
            for k in 0..3 {
                sum += vertex[k] * m2[k][j];
            }
            temp_result[j] = sum;
        }
        result.push(temp_result);
    }
    return result;
}

pub fn rot_mat_x(angle: f32) -> [types::V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [1.0, 0.0, 0.0],
        [0.0, ang_rad.cos(), -ang_rad.sin()],
        [0.0, ang_rad.sin(), ang_rad.cos()],
    ];
}

pub fn rot_mat_y(angle: f32) -> [types::V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), 0.0, ang_rad.sin()],
        [0.0, 1.0, 0.0],
        [-ang_rad.sin(), 0.0, ang_rad.cos()],
    ];
}

pub fn rot_mat_z(angle: f32) -> [types::V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), -ang_rad.sin(), 0.0],
        [ang_rad.sin(), ang_rad.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ];
}
