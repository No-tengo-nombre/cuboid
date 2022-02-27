pub fn vec4_to_v4<T: Copy>(vec4: &Vec<T>) -> [T; 4] {
    return [vec4[0], vec4[1], vec4[2], vec4[3]];
}

pub fn vec3_to_v3<T: Copy>(vec3: &Vec<T>) -> [T; 3] {
    return [vec3[0], vec3[1], vec3[2]];
}
