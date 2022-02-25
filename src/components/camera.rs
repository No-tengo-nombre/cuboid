use crate::utils::math::vector;
use crate::utils::types::V3;

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

    pub fn set_target(&mut self, target: &V3) {
        self._direction = Camera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        return vector::normalize_v3(&vector::sub_v3(target, position));
    }
}
