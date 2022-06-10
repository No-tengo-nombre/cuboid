use crate::utils::types::{V3, V4};

pub fn vec4_to_v4<T: Copy>(vec4: &Vec<T>) -> [T; 4] {
    return [vec4[0], vec4[1], vec4[2], vec4[3]];
}

pub fn vec3_to_v3<T: Copy>(vec3: &Vec<T>) -> [T; 3] {
    return [vec3[0], vec3[1], vec3[2]];
}

pub fn v3_to_v4(v: &V3) -> V4 {
    return [v[0], v[1], v[2], 1.0];
}

pub fn v4_to_v3(v: &V4) -> V3 {
    let w = v[3];
    return [v[0] / w, v[1] / w, v[2] / w];
}

pub fn mat3_to_mat4(mat3: &[V3; 3]) -> [V4; 4] {
    let mut result = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = mat3[i][j];
        }
    }

    return result;
}
