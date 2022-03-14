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

    pub fn get_translate(&self) -> &TransformTranslate {
        return &self._translate;
    }

    pub fn get_rotate(&self) -> &TransformRotate {
        return &self._rotate;
    }
    pub fn get_scale(&self) -> &TransformScale {
        return &self._scale;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Transform components |=====================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

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
}

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
}

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
}
