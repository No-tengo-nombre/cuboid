use crate::utils::types::{V3, V4, V6};

///////////////////////////////////////////////////////////////////////////////////////////////////
//|=================================| Vector operations |=======================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn norm_v3(target: &V3) -> f32 {
    let x = target[0];
    let y = target[1];
    let z = target[2];
    return (x * x + y * y + z * z).sqrt();
}

pub fn normalize_v3(target: &V3) -> V3 {
    let x = target[0];
    let y = target[1];
    let z = target[2];
    let length = norm_v3(target);
    return [x / length, y / length, z / length];
}

pub fn sub_v3(a: &V3, b: &V3) -> V3 {
    let xa = a[0];
    let ya = a[1];
    let za = a[2];
    let xb = b[0];
    let yb = b[1];
    let zb = b[2];
    return [xa - xb, ya - yb, za - zb];
}

pub fn cross_v3(a: &V3, b: &V3) -> V3 {
    let xa = a[0];
    let ya = a[1];
    let za = a[2];
    let xb = b[0];
    let yb = b[1];
    let zb = b[2];
    return [ya * zb - za * yb, za * xb - xa * zb, xa * yb - ya * xb];
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|===============================| Matrix multiplication |=====================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mat4_mul4(m1: &[V4; 4], m2: &[V4; 4]) -> [V4; 4] {
    let mut result = eye4();
    for i in 0..4 {
        let vertex = m1[i];
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += vertex[k] * m2[k][j];
            }
            result[i][j] = sum;
        }
    }
    return result;
}

pub fn mat4_mul_v4(m1: &[V4; 4], v: &V4) -> V4 {
    let mut result = [0.0, 0.0, 0.0, 0.0];
    for i in 0..4 {
        let mut sum = 0.0;
        for k in 0..4 {
            sum += m1[i][k] * v[k];
        }
        result[i] = sum;
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

pub fn mat3_mul3(m1: &[V3; 3], m2: &[V3; 3]) -> [V3; 3] {
    let mut result = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
    for i in 0..3 {
        let vertex = m1[i];
        for j in 0..3 {
            let mut sum = 0.0;
            for k in 0..3 {
                sum += vertex[k] * m2[k][j];
            }
            result[i][j] = sum;
        }
    }
    return result;
}

pub fn mat3_mul_v3(m1: &[V3], v: &V3) -> V3 {
    let mut result = [0.0, 0.0, 0.0];
    for i in 0..m1.len() {
        let vertex = m1[i];
        let temp_result = v[0] * vertex[0] + v[1] * vertex[1] + v[2] * vertex[2];
        result[i] = temp_result;
    }
    return result;
}

pub fn transpose4x4(m1: &Vec<V4>) -> Vec<V4> {
    let mut result = Vec::<V4>::new();
    for j in 0..4 {
        let mut temp_result: V4 = [0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            temp_result[i] = m1[i][j];
        }
        result.push(temp_result);
    }
    return result;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Important matrices |=======================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn eye3() -> [V3; 3] {
    return [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];
}

pub fn eye4() -> [V4; 4] {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn rot_mat3_x(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [1.0, 0.0, 0.0],
        [0.0, ang_rad.cos(), -ang_rad.sin()],
        [0.0, ang_rad.sin(), ang_rad.cos()],
    ];
}

pub fn rot_mat3_y(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), 0.0, ang_rad.sin()],
        [0.0, 1.0, 0.0],
        [-ang_rad.sin(), 0.0, ang_rad.cos()],
    ];
}

pub fn rot_mat3_z(angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    return [
        [ang_rad.cos(), -ang_rad.sin(), 0.0],
        [ang_rad.sin(), ang_rad.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ];
}

pub fn rot_mat3(axis: &V3, angle: f32) -> [V3; 3] {
    let ang_rad = angle.to_radians();
    let x = axis[0];
    let y = axis[1];
    let z = axis[2];
    let c = ang_rad.cos();
    let s = ang_rad.sin();
    return [
        [
            (1.0 - c) * x * x + c,
            (1.0 - c) * x * y - s * z,
            (1.0 - c) * x * z + s * y,
        ],
        [
            (1.0 - c) * x * y + s * z,
            (1.0 - c) * y * y + c,
            (1.0 - c) * y * z - s * x,
        ],
        [
            (1.0 - c) * x * z - s * y,
            (1.0 - c) * y * z + s * x,
            (1.0 - c) * z * z + c,
        ],
    ];
}

pub fn scale_mat3(x: f32, y: f32, z: f32) -> [V3; 3] {
    return [
        [x, 0.0, 0.0],
        [0.0, y, 0.0],
        [0.0, 0.0, z],
    ];
}

pub fn look_at(position: &V3, up: &V3, direction: &V3) -> [V4; 4] {
    let right = normalize_v3(&cross_v3(&up, &direction));
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
        [0.0, 0.0, 1.0, -pz],
        [0.0, 0.0, 0.0, 1.0],
    ];
    return mat4_mul4(&matrix1, &matrix2);
}

pub fn ortho(xmin: f32, xmax: f32, ymin: f32, ymax: f32, zmin: f32, zmax: f32) -> [V4; 4] {
    let rml = xmax - xmin;
    let tmb = ymax - ymin;
    let fmn = zmax - zmin;
    let rpl = xmax + xmin;
    let tpb = ymax + ymin;
    let fpn = zmax + zmin;
    return [
        [2.0 / rml, 0.0, 0.0, -rpl / rml],
        [0.0, 2.0 / tmb, 0.0, -tpb / tmb],
        [0.0, 0.0, -2.0 / fmn, -fpn / fmn],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn perspective(xmin: f32, xmax: f32, ymin: f32, ymax: f32, zmin: f32, zmax: f32) -> [V4; 4] {
    let rml = xmax - xmin;
    let tmb = ymax - ymin;
    let fmn = zmax - zmin;
    let rpl = xmax + xmin;
    let tpb = ymax + ymin;
    let fpn = zmax + zmin;
    return [
        [2.0 * zmin / rml, 0.0, 0.0, 0.0],
        [0.0, 2.0 * zmin / tmb, 0.0, 0.0],
        [rpl / rml, tpb / tmb, -fpn / fmn, -1.0],
        [0.0, 0.0, -2.0 * zmax * zmin / fmn, 0.0],
    ];
}
