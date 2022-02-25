/// A three-dimensional vertex, with coordinates x, y and z.
pub type V3 = [f32; 3];

pub type V4 = [f32; 4];

/// A six-dimensional vertex, used to indicate three spatial coordinates along
/// with 3 values for the color.
pub type V6 = [f32; 6];


// pub struct M3<'a> {
//     _values: &'a [V3],
// }

// pub struct M6<'a> {
//     _values: &'a [V6],
// }

// impl<'a> M3<'a> {
//     pub fn column(&self, i: usize) -> [f32; 3] {
//         let mut result: [f32; 3] = [0.0, 0.0, 0.0];
//         for j in 0..self.size() {
//             result[j] = self._values[j][i];
//         }
//         return result;
//     }

//     pub fn size(&self) -> usize {
//         return self._values.len();
//     }
// }

// impl<'a> M6<'a> {
//     pub fn size(&self) -> usize {
//         return self._values.len();
//     }

//     pub fn get_vertex(&self, i: usize) -> V6 {
//         return self._values[i];
//     }

//     pub fn get(&self, i: usize, j: usize) -> f32 {
//         return self._values[i][j];
//     }

//     pub fn mul3(&self, target: &M3) {
//         for i in 0..self.size() {
//             let vertex = self.get_vertex(i);
//             let result: V6 = [0.0, 0.0, 0.0, vertex[3], vertex[4], vertex[5]];
//             for j in 0..3 {

//             }
//         }
//     }
// }
