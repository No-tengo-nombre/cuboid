// Using the 10 first digits of pi
pub const PI: f32 = 3.1415926535;
const PI_O_180: f32 = PI / 180.0;


/// Transforms degrees to radians
pub fn d2r(degrees: f32) -> f32 {
    return PI_O_180 * degrees;
}
