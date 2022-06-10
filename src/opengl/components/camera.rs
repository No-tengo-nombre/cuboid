use crate::utils::conversions;
use crate::utils::math::linalg;
use crate::utils::types::{V3, V4};
use crate::opengl::{buffers::UBO, Transform, Transformable};

pub trait Camera {
    fn get_position(&self) -> V3;
    fn get_direction(&self) -> V3;
    fn get_up(&self) -> V3;
    fn get_right(&self) -> V3;
    fn get_transform(&self) -> Vec<V4>;
    fn get_applied_transform(&self) -> Transform;
    fn update(&mut self, new_pos: &V3, new_dir: &V3, new_up: &V3);
    fn get_ubo(&self) -> UBO;
    fn set_ubo(&mut self, ubo: UBO);

    /*
    The following functions (update_ubo and make_ubo) are not meant to be overwritten. Instead,
    they define the default behaviour for the camera's uniform, which is bound to the index 0
    and contains the data associated with the MVP matrix.
    */

    /// Update the UBO
    fn update_ubo(&self) {
        self.get_ubo().buffer_data(0, &self.get_transform());
    }

    /// Makes an UBO
    fn make_ubo() -> UBO {
        let camera_ubo = UBO::new().size(64).build();
        camera_ubo.bind_index(0);
        return camera_ubo;
    }
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
    pub _position: V3,
    pub _direction: V3,
    pub _up: V3,
    pub _right: V3,
    pub _ubo: UBO,
    pub _transform: Transform,
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

    fn get_ubo(&self) -> UBO {
        return self._ubo;
    }

    fn set_ubo(&mut self, ubo: UBO) {
        self._ubo = ubo;
    }

    fn get_applied_transform(&self) -> Transform {
        return self._transform;
    }

    fn get_transform(&self) -> Vec<V4> {
        let look = linalg::look_at(
            &self._position,
            &self._up,
            &self._direction,
            // &self.right,
        );
        let orth_view = linalg::ortho(
            self._xmin, self._xmax, self._ymin, self._ymax, self._zmin, self._zmax,
        );
        return linalg::mat4_mul4(
            &conversions::vec4_to_v4(&orth_view),
            &conversions::vec4_to_v4(&look),
        );
    }

    fn update(&mut self, new_pos: &V3, new_dir: &V3, new_up: &V3) {
        self._position = *new_pos;
        self._direction = *new_dir;
        self._up = *new_up;
        // self.right = linalg::cross_v3(&self.direction, &self.up);
        self._right = linalg::cross_v3(&self._up, &self._direction);
        self.update_ubo();
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
        let new_direction = linalg::normalize_v3(direction);
        return OrthoCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: linalg::normalize_v3(up),
            // _right: linalg::normalize_v3(&linalg::cross_v3(&new_direction, &up)),
            _right: linalg::normalize_v3(&linalg::cross_v3(&up, &new_direction)),
            _ubo: OrthoCamera::make_ubo(),
            _transform: Transform::new(),
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
            _up: linalg::normalize_v3(up),
            // _right: linalg::normalize_v3(&linalg::cross_v3(&new_direction, &up)),
            _right: linalg::normalize_v3(&linalg::cross_v3(&up, &new_direction)),
            _ubo: OrthoCamera::make_ubo(),
            _transform: Transform::new(),
        };
    }

    pub fn set_target(&mut self, target: &V3) {
        self._direction = OrthoCamera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        return linalg::normalize_v3(&linalg::sub_v3(position, target));
    }
}

impl Transformable for OrthoCamera {
    fn get_trans(&self) -> Transform {
        return self.get_applied_transform();
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
    pub _position: V3,
    pub _direction: V3,
    pub _up: V3,
    pub _right: V3,
    pub _ubo: UBO,
    pub _transform: Transform,
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

    fn get_ubo(&self) -> UBO {
        return self._ubo;
    }

    fn set_ubo(&mut self, ubo: UBO) {
        self._ubo = ubo;
    }

    fn get_applied_transform(&self) -> Transform {
        return self._transform;
    }

    fn get_transform(&self) -> Vec<V4> {
        let look = linalg::look_at(
            &self._position,
            &self._up,
            &self._direction,
            // &self.right,
        );
        let persp_view = linalg::perspective(
            self._xmin, self._xmax, self._ymin, self._ymax, self._zmin, self._zmax,
        );
        return linalg::mat4_mul4(
            &conversions::vec4_to_v4(&persp_view),
            &conversions::vec4_to_v4(&look),
        );
    }

    fn update(&mut self, new_pos: &V3, new_dir: &V3, new_up: &V3) {
        self._position = *new_pos;
        // self.direction = *new_dir;
        // self.up = *new_up;
        self._direction = linalg::normalize_v3(new_dir);
        self._up = linalg::normalize_v3(new_up);
        // self.right = linalg::cross_v3(&self.direction, &self.up);
        self._right = linalg::cross_v3(&self._up, &self._direction);
        self.update_ubo();
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
        let new_direction = linalg::normalize_v3(direction);
        return PerspectiveCamera {
            _xmin: xmin,
            _xmax: xmax,
            _ymin: ymin,
            _ymax: ymax,
            _zmin: zmin,
            _zmax: zmax,
            _position: *position,
            _direction: new_direction,
            _up: linalg::normalize_v3(up),
            // _right: linalg::normalize_v3(&linalg::cross_v3(&new_direction, &up)),
            _right: linalg::normalize_v3(&linalg::cross_v3(&up, &new_direction)),
            _ubo: PerspectiveCamera::make_ubo(),
            _transform: Transform::new(),
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
            _up: linalg::normalize_v3(up),
            // _right: linalg::normalize_v3(&linalg::cross_v3(&new_direction, &up)),
            _right: linalg::normalize_v3(&linalg::cross_v3(&up, &new_direction)),
            _ubo: PerspectiveCamera::make_ubo(),
            _transform: Transform::new(),
        };
    }

    pub fn set_target(&mut self, target: &V3) {
        self._direction = PerspectiveCamera::dir_from_target(&self._position, target);
    }

    fn dir_from_target(position: &V3, target: &V3) -> V3 {
        return linalg::normalize_v3(&linalg::sub_v3(target, position));
    }
}
