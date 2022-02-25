use crate::utils::math::{linalg, vector};
use crate::utils::types::{V3, V4};

pub trait Camera {
    fn get_position(&self) -> V3;
    fn get_direction(&self) -> V3;
    fn get_up(&self) -> V3;
    fn get_right(&self) -> V3;
    fn look_at(&self) -> Vec<V4>;
}

pub struct OrthoCamera {
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
        return linalg::look_at(
            &self.get_position(),
            &self.get_up(),
            &self.get_direction(),
            &self.get_right(),
        );
    }
}

impl OrthoCamera {
    pub fn new(position: &V3, direction: &V3, up: &V3) -> OrthoCamera {
        let new_direction = vector::normalize_v3(direction);
        return OrthoCamera {
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn new_from_target(position: &V3, target: &V3, up: &V3) -> OrthoCamera {
        let new_direction = OrthoCamera::dir_from_target(position, target);
        return OrthoCamera {
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
        return vector::normalize_v3(&vector::sub_v3(target, position));
    }
}
