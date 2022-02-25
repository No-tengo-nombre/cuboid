pub fn vec4_to_v4<T: Copy>(vec4: &Vec<T>) -> [T; 4] {
    return [vec4[0], vec4[1], vec4[2], vec4[3]];
}
