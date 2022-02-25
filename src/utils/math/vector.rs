use crate::utils::types::V3;

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
    return [
        ya * zb - za * yb,
        za * xb - xa * zb,
        xa * yb - ya * xb,
    ];
}
