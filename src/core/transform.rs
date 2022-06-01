use crate::utils::types::V4;
use std::ops;

#[derive(Copy, Clone)]
pub struct Transform {
    _translate: TransformTranslate,
    _rotate: TransformRotate,
    _scale: TransformScale,
}

#[derive(Copy, Clone)]
pub struct TransformTranslate {
    _x: f32,
    _y: f32,
    _z: f32,
}

#[derive(Copy, Clone)]
pub struct TransformRotate {
    _theta: f32,
    _phi: f32,
}

#[derive(Copy, Clone)]
pub struct TransformScale {
    _x: f32,
    _y: f32,
    _z: f32,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|===============================| Transform implementation |==================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

impl Transform {
    pub fn new() -> Transform {
        return Transform {
            _translate: TransformTranslate::new(),
            _rotate: TransformRotate::new(),
            _scale: TransformScale::new(),
        };
    }

    pub fn new_value(
        trans: TransformTranslate,
        rot: TransformRotate,
        scl: TransformScale,
    ) -> Transform {
        return Transform {
            _translate: trans,
            _rotate: rot,
            _scale: scl,
        };
    }

    pub fn add(&mut self, transform: &Transform) {
        self._translate += *transform.get_translate();
        self._rotate += *transform.get_rotate();
        self._scale += *transform.get_scale();
    }

    pub fn get_translate(&self) -> &TransformTranslate {
        return &self._translate;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self._translate += TransformTranslate::new_value(x, y, z);
    }

    pub fn get_rotate(&self) -> &TransformRotate {
        return &self._rotate;
    }

    pub fn rotate(&mut self, theta: f32, phi: f32) {
        self._rotate += TransformRotate::new_value(theta, phi);
    }

    pub fn get_scale(&self) -> &TransformScale {
        return &self._scale;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self._scale += TransformScale::new_value(x, y, z);
    }
}

impl ops::Add<Transform> for Transform {
    type Output = Transform;

    fn add(self, rhs: Transform) -> Transform {
        return Transform::new_value(
            *self.get_translate() + *rhs.get_translate(),
            *self.get_rotate() + *rhs.get_rotate(),
            *self.get_scale() + *rhs.get_scale(),
        );
    }
}

impl ops::AddAssign<Transform> for Transform {
    fn add_assign(&mut self, rhs: Transform) {
        let result = *self + rhs;
        self._translate = *result.get_translate();
        self._rotate = *result.get_rotate();
        self._scale = *result.get_scale();
    }
}

impl ops::Sub<Transform> for Transform {
    type Output = Transform;

    fn sub(self, rhs: Transform) -> Transform {
        return Transform::new_value(
            *self.get_translate() - *rhs.get_translate(),
            *self.get_rotate() - *rhs.get_rotate(),
            *self.get_scale() - *rhs.get_scale(),
        );
    }
}

impl ops::SubAssign<Transform> for Transform {
    fn sub_assign(&mut self, rhs: Transform) {
        let result = *self - rhs;
        self._translate = *result.get_translate();
        self._rotate = *result.get_rotate();
        self._scale = *result.get_scale();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Transform components |=====================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////// TransformTranslate ///////////////////////////////////////////

impl TransformTranslate {
    pub fn new() -> TransformTranslate {
        return TransformTranslate::new_value(0.0, 0.0, 0.0);
    }

    pub fn new_value(x: f32, y: f32, z: f32) -> TransformTranslate {
        return TransformTranslate {
            _x: x,
            _y: y,
            _z: z,
        };
    }

    pub fn get_x(&self) -> f32 {
        return self._x;
    }

    pub fn get_y(&self) -> f32 {
        return self._y;
    }

    pub fn get_z(&self) -> f32 {
        return self._z;
    }

    pub fn mat4(&self) -> [V4; 4] {
        return [
            [1.0, 0.0, 0.0, self._x],
            [0.0, 1.0, 0.0, self._y],
            [0.0, 0.0, 1.0, self._z],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }
}

impl ops::Add<TransformTranslate> for TransformTranslate {
    type Output = TransformTranslate;

    fn add(self, rhs: TransformTranslate) -> TransformTranslate {
        return TransformTranslate::new_value(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        );
    }
}

impl ops::AddAssign<TransformTranslate> for TransformTranslate {
    fn add_assign(&mut self, rhs: TransformTranslate) {
        let out_transform = *self + rhs;
        self._x = out_transform.get_x();
        self._y = out_transform.get_y();
        self._z = out_transform.get_z();
    }
}

impl ops::Sub<TransformTranslate> for TransformTranslate {
    type Output = TransformTranslate;

    fn sub(self, rhs: TransformTranslate) -> TransformTranslate {
        return TransformTranslate::new_value(
            self.get_x() - rhs.get_x(),
            self.get_y() - rhs.get_y(),
            self.get_z() - rhs.get_z(),
        );
    }
}

impl ops::SubAssign<TransformTranslate> for TransformTranslate {
    fn sub_assign(&mut self, rhs: TransformTranslate) {
        let out_transform = *self - rhs;
        self._x = out_transform.get_x();
        self._y = out_transform.get_y();
        self._z = out_transform.get_z();
    }
}

//////////////////////////////////// TransformRotate //////////////////////////////////////////////

impl TransformRotate {
    pub fn new() -> TransformRotate {
        return TransformRotate::new_value(0.0, 0.0);
    }

    pub fn new_value(theta: f32, phi: f32) -> TransformRotate {
        return TransformRotate {
            _theta: theta,
            _phi: phi,
        };
    }

    pub fn get_theta(&self) -> f32 {
        return self._theta;
    }

    pub fn get_phi(&self) -> f32 {
        return self._phi;
    }
}

impl ops::Add<TransformRotate> for TransformRotate {
    type Output = TransformRotate;

    fn add(self, rhs: TransformRotate) -> TransformRotate {
        return TransformRotate::new_value(
            self._theta + rhs.get_theta(),
            self._phi + rhs.get_phi(),
        );
    }
}

impl ops::AddAssign<TransformRotate> for TransformRotate {
    fn add_assign(&mut self, rhs: TransformRotate) {
        let output = *self + rhs;
        self._theta = output.get_theta();
        self._phi = output.get_phi();
    }
}

impl ops::Sub<TransformRotate> for TransformRotate {
    type Output = TransformRotate;

    fn sub(self, rhs: TransformRotate) -> TransformRotate {
        return TransformRotate::new_value(
            self._theta - rhs.get_theta(),
            self._phi - rhs.get_phi(),
        );
    }
}

impl ops::SubAssign<TransformRotate> for TransformRotate {
    fn sub_assign(&mut self, rhs: TransformRotate) {
        let output = *self - rhs;
        self._theta = output.get_theta();
        self._phi = output.get_phi();
    }
}

//////////////////////////////////// TransformScale ///////////////////////////////////////////////

impl TransformScale {
    pub fn new() -> TransformScale {
        return TransformScale::new_value(1.0, 1.0, 1.0);
    }

    pub fn new_value(x: f32, y: f32, z: f32) -> TransformScale {
        return TransformScale {
            _x: x,
            _y: y,
            _z: z,
        };
    }

    pub fn get_x(&self) -> f32 {
        return self._x;
    }

    pub fn get_y(&self) -> f32 {
        return self._y;
    }

    pub fn get_z(&self) -> f32 {
        return self._z;
    }
}

impl ops::Add<TransformScale> for TransformScale {
    type Output = TransformScale;

    fn add(self, rhs: TransformScale) -> TransformScale {
        return TransformScale::new_value(
            self._x + rhs.get_x(),
            self._y + rhs.get_y(),
            self._z + rhs.get_z(),
        );
    }
}

impl ops::AddAssign<TransformScale> for TransformScale {
    fn add_assign(&mut self, rhs: TransformScale) {
        let output = *self + rhs;
        self._x = output.get_x();
        self._y = output.get_y();
        self._z = output.get_z();
    }
}

impl ops::Sub<TransformScale> for TransformScale {
    type Output = TransformScale;

    fn sub(self, rhs: TransformScale) -> TransformScale {
        return TransformScale::new_value(
            self._x - rhs.get_x(),
            self._y - rhs.get_y(),
            self._z - rhs.get_z(),
        );
    }
}

impl ops::SubAssign<TransformScale> for TransformScale {
    fn sub_assign(&mut self, rhs: TransformScale) {
        let output = *self - rhs;
        self._x = output.get_x();
        self._y = output.get_y();
        self._z = output.get_z();
    }
}
