/// A 2x3 matrix is represented as float[6].
pub struct Transform {
    pub m: [f32; 6],
}

impl Transform {
    /// Sets the transform to identity matrix.
    pub fn set_identity(&mut self) {
        unimplemented!();
    }

    /// Sets the transform to translation matrix matrix.
    pub fn set_translate(&mut self, tx: f32, ty: f32) {
        unimplemented!();
    }

    /// Sets the transform to scale matrix.
    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        unimplemented!();
    }

    /// Sets the transform to rotate matrix. Angle is specified
    /// in radians.
    pub fn set_rotate(&mut self, a: f32) {
        unimplemented!();
    }

    /// Sets the transform to skew-x matrix. Angle is specified
    /// in radians.
    pub fn set_skew_y(&mut self, y: f32) {
        unimplemented!();
    }

    /// Sets the transform to skew-y matrix. Angle is specified
    /// in radians.
    pub fn set_skew_x(&mut self, x: f32) {
        unimplemented!();
    }

    /// Sets the transform to the result of multiplication of two
    /// transforms, of A = A*B.
    pub fn set_multiply(&mut self, src: &Transform) {
        unimplemented!();
    }

    /// Sets the transform to the result of multiplication of two
    /// transforms, of A = B*A.
    pub fn set_premultiply(&mut self, src: &Transform) {
        unimplemented!();
    }

    /// Computes the transformation's inverse and returns it.
    pub fn invert(&self) -> Option<Transform> {
        unimplemented!();
    }

    /// Transform a point by given transform.
    pub fn point(&self, srcx: f32, srcy: f32) -> (f32, f32) {
        unimplemented!();
    }
}
