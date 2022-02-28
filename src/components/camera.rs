use crate::utils::conversions;
use crate::utils::math::{linalg, vector};
use crate::utils::types::{V3, V4};

pub trait Camera {
    fn get_position(&self) -> V3;
    fn get_direction(&self) -> V3;
    fn get_up(&self) -> V3;
    fn get_right(&self) -> V3;
    fn look_at(&self) -> Vec<V4>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Orthogonal camera |========================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OrthoCamera {
    _xmin: f32,
    _xmax: f32,
    _ymin: f32,
    _ymax: f32,
    _zmin: f32,
    _zmax: f32,
    _position: V3,
    _direction: V3,
    _up: V3,
    _right: V3,
}

impl Camera for OrthoCamera {
    fn get_position(&self) -> V3 {
        return self._position;
    }

    fn get_up(&self) -> V3 {
        return self._up;
    }

    fn get_right(&self) -> V3 {
        return self._right;
    }

    fn get_direction(&self) -> V3 {
        return self._direction;
    }

    fn look_at(&self) -> Vec<V4> {
        let look = linalg::look_at(
            &self.get_position(),
            &self.get_up(),
            &self.get_direction(),
            &self.get_right(),
        );
        let orth_view = linalg::ortho(
            self._xmin, self._xmax, self._ymin, self._ymax, self._zmin, self._zmax,
        );
        return linalg::mat4_mul4(
            &conversions::vec4_to_v4(&orth_view),
            &conversions::vec4_to_v4(&look),
        );
    }
}

impl OrthoCamera {
    pub fn new(
        position: &V3,
        direction: &V3,
        up: &V3,
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> OrthoCamera {
        let new_direction = vector::normalize_v3(direction);
        return OrthoCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn new_from_target(
        position: &V3,
        target: &V3,
        up: &V3,
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> OrthoCamera {
        let new_direction = OrthoCamera::dir_from_target(position, target);
        return OrthoCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn set_target(&mut self, target: &V3) {
        self._direction = OrthoCamera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        // return vector::normalize_v3(&vector::sub_v3(target, position));
        return vector::normalize_v3(&vector::sub_v3(position, target));
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Perspective camera |=======================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct PerspectiveCamera {
    _xmin: f32,
    _xmax: f32,
    _ymin: f32,
    _ymax: f32,
    _zmin: f32,
    _zmax: f32,
    _position: V3,
    _direction: V3,
    _up: V3,
    _right: V3,
}

impl Camera for PerspectiveCamera {
    fn get_position(&self) -> V3 {
        return self._position;
    }

    fn get_up(&self) -> V3 {
        return self._up;
    }

    fn get_right(&self) -> V3 {
        return self._right;
    }

    fn get_direction(&self) -> V3 {
        return self._direction;
    }

    fn look_at(&self) -> Vec<V4> {
        let look = linalg::look_at(
            &self.get_position(),
            &self.get_up(),
            &self.get_direction(),
            &self.get_right(),
        );
        let persp_view = linalg::perspective(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        return linalg::mat4_mul4(
            &conversions::vec4_to_v4(&persp_view),
            &conversions::vec4_to_v4(&look),
        );
    }
}

impl PerspectiveCamera {
    pub fn new(
        position: &V3,
        direction: &V3,
        up: &V3,
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> PerspectiveCamera {
        let new_direction = vector::normalize_v3(direction);
        return PerspectiveCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn new_from_target(
        position: &V3,
        target: &V3,
        up: &V3,
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> PerspectiveCamera {
        let new_direction = PerspectiveCamera::dir_from_target(position, target);
        return PerspectiveCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn set_target(&mut self, target: &V3) {
        self._direction = PerspectiveCamera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        return vector::normalize_v3(&vector::sub_v3(target, position));
    }
}
