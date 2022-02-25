use crate::utils::math::{linalg, vector};
use crate::utils::types::{V3, V4};

pub struct Camera {
    _position: V3,
    _direction: V3,
    _up: V3,
    _right: V3,
}

impl Camera {
    pub fn new(position: &V3, direction: &V3, up: &V3) -> Camera {
        let new_direction = vector::normalize_v3(direction);
        return Camera {
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn new_from_target(position: &V3, target: &V3, up: &V3) -> Camera {
        let new_direction = Camera::dir_from_target(position, target);
        return Camera {
            _position: *position,
            _direction: new_direction,
            _up: vector::normalize_v3(up),
            _right: vector::normalize_v3(&vector::cross_v3(up, &new_direction)),
        };
    }

    pub fn get_position(&self) -> V3 {
        return self._position;
    }

    pub fn get_up(&self) -> V3 {
        return self._up;
    }

    pub fn get_right(&self) -> V3 {
        return self._right;
    }

    pub fn get_direction(&self) -> V3 {
        return self._direction;
    }

    pub fn look_at(&self) -> Vec<V4> {
        return linalg::look_at(
            &self.get_position(),
            &self.get_up(),
            &self.get_direction(),
            &self.get_right(),
        );
    }

    pub fn set_target(&mut self, target: &V3) {
        self._direction = Camera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        return vector::normalize_v3(&vector::sub_v3(target, position));
    }
}
