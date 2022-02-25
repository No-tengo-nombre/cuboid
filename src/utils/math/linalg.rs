use crate::utils::types::{V3, V4, V6};


pub fn mat4_mul4(m1: &[V4; 4], m2: &[V4; 4]) -> Vec<V4> {
    let mut result = Vec::<V4>::new();
    for i in 0..m1.len() {
        let vertex = m1[i];
        let mut temp_result: V4 = [0.0, 0.0, 0.0, 0.0];
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += vertex[k] * m2[k][j];
            }
            temp_result[j] = sum;
        }
        result.push(temp_result);
    }
    return result;
}

pub fn mat6_mul3(m1: &[V6], m2: &[V3; 3]) -> Vec<V6> {
    let mut result = Vec::<V6>::new();
    for i in 0..m1.len() {
        let vertex = m1[i];
        let mut temp_result: V6 = [0.0, 0.0, 0.0, vertex[3], vertex[4], vertex[5]];
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

// pub fn mat6_mul3(m1: &Vec<V6>, m2: &[V3; 3]) -> Vec<V6> {
//     let mut result = Vec::<V6>::new();
//     for i in 0..m1.len() {
//         let vertex = m1[i];
//         let mut temp_result: V6 = [0.0, 0.0, 0.0, vertex[3], vertex[4], vertex[5]];
//         for j in 0..3 {
//             let mut sum = 0.0;
//             for k in 0..3 {
//                 sum += vertex[k] * m2[k][j];
//             }
//             temp_result[j] = sum;
//         }
//         result.push(temp_result);
//     }
//     return result;
// }

// pub fn mat6_mul3<T: Index<usize>>(m1: &[T], m2: &[V3; 3]) -> Vec<T> {
//     let mut result = Vec::<T>::new();
//     for i in 0..m1.len() {
//         let vertex = m1[i];
//         let mut temp_result: T = [0.0, 0.0, 0.0, vertex[3], vertex[4], vertex[5]];
//         for j in 0..3 {
//             let mut sum = 0.0;
//             for k in 0..3 {
//                 sum += vertex[k] * m2[k][j];
//             }
//             temp_result[j] = sum;
//         }
//         result.push(temp_result);
//     }
//     return result;
// }

pub fn rot_mat_x(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [1.0, 0.0, 0.0],
        [0.0, ang_rad.cos(), -ang_rad.sin()],
        [0.0, ang_rad.sin(), ang_rad.cos()],
    ];
}

pub fn rot_mat_y(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), 0.0, ang_rad.sin()],
        [0.0, 1.0, 0.0],
        [-ang_rad.sin(), 0.0, ang_rad.cos()],
    ];
}

pub fn rot_mat_z(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), -ang_rad.sin(), 0.0],
        [ang_rad.sin(), ang_rad.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ];
}

pub fn look_at(position: &V3, up: &V3, direction: &V3, right: &V3) -> Vec<V4> {
    let rx = right[0];
    let ry = right[1];
    let rz = right[2];
    let ux = up[0];
    let uy = up[1];
    let uz = up[2];
    let dx = direction[0];
    let dy = direction[1];
    let dz = direction[2];
    let px = position[0];
    let py = position[1];
    let pz = position[2];
    let matrix1 = [
        [rx, ry, rz, 0.0],
        [ux, uy, uz, 0.0],
        [dx, dy, dz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let matrix2 = [
        [1.0, 0.0, 0.0, -px],
        [0.0, 1.0, 0.0, -py],
        [0.0, 0.0, 1.0, -px],
        [0.0, 0.0, 0.0, 1.0],
    ];
    return mat4_mul4(&matrix1, &matrix2);
}
