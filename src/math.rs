
/// A 2x3 matrix is represented as float[6].
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub m: [f32; 6],
}

impl Transform {
    /// Sets the transform to identity matrix.
    pub fn set_identity(&mut self) {
        self.m[0] = 1.0;
        self.m[1] = 0.0;
    	self.m[2] = 0.0;
        self.m[3] = 1.0;
    	self.m[4] = 0.0;
        self.m[5] = 0.0;
    }

    /// Sets the transform to translation matrix matrix.
    pub fn set_translate(&mut self, tx: f32, ty: f32) {
        self.m[0] = 1.0;
        self.m[1] = 0.0;
    	self.m[2] = 0.0;
        self.m[3] = 1.0;
    	self.m[4] = tx;
        self.m[5] = ty;
    }

    /// Sets the transform to scale matrix.
    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        self.m[0] = sx;
        self.m[1] = 0.0;
    	self.m[2] = 0.0;
        self.m[3] = sy;
    	self.m[4] = 0.0;
        self.m[5] = 0.0;
    }

    /// Sets the transform to rotate matrix. Angle is specified
    /// in radians.
    pub fn set_rotate(&mut self, a: f32) {
        let cs = f32::cos(a);
        let sn = f32::sin(a);
    	self.m[0] = cs;
        self.m[1] = sn;
    	self.m[2] = -sn;
        self.m[3] = cs;
    	self.m[4] = 0.0;
        self.m[5] = 0.0;
    }

    /// Sets the transform to skew-x matrix. Angle is specified
    /// in radians.
    pub fn set_skew_y(&mut self, y: f32) {
        self.m[0] = 1.0;
        self.m[1] = 0.0;
    	self.m[2] = f32::tan(y);
        self.m[3] = 1.0;
    	self.m[4] = 0.0;
        self.m[5] = 0.0;
    }

    /// Sets the transform to skew-y matrix. Angle is specified
    /// in radians.
    pub fn set_skew_x(&mut self, x: f32) {
        self.m[0] = 1.0;
        self.m[1] = f32::tan(x);
    	self.m[2] = 0.0;
        self.m[3] = 1.0;
    	self.m[4] = 0.0;
        self.m[5] = 0.0;
    }

    /// Sets the transform to the result of multiplication of two
    /// transforms, of A = A*B.
    pub fn set_multiply(&mut self, src: &Transform) {
        let t0 = self.m[0] * src.m[0] + self.m[1] * src.m[2];
    	let t2 = self.m[2] * src.m[0] + self.m[3] * src.m[2];
    	let t4 = self.m[4] * src.m[0] + self.m[5] * src.m[2] + src.m[4];
    	self.m[1] = self.m[0] * src.m[1] + self.m[1] * src.m[3];
    	self.m[3] = self.m[2] * src.m[1] + self.m[3] * src.m[3];
    	self.m[5] = self.m[4] * src.m[1] + self.m[5] * src.m[3] + src.m[5];
    	self.m[0] = t0;
    	self.m[2] = t2;
    	self.m[4] = t4;
    }

    /// Sets the transform to the result of multiplication of two
    /// transforms, of A = B*A.
    pub fn set_premultiply(&mut self, src: &Transform) {
        let myself = *self;
        *self = *src;
        self.set_multiply(&myself);
    }

    /// Computes the transformation's inverse and returns it.
    pub fn invert(&self) -> Option<Transform> {
        let det: f64 = (self.m[0] as f64) * (self.m[3] as f64) - (self.m[2] as f64) * (self.m[1] as f64);
    	if det > -1e-6 && det < 1e-6 {
    		None
    	} else {
            // NB we are about to set every value, and 2x3 matrices don't change size.
            let mut result: Transform = unsafe { ::std::mem::uninitialized() };
        	let invdet: f64 = 1.0 / det;
        	result.m[0] = ((self.m[3] as f64) * invdet) as f32;
        	result.m[2] = (-(self.m[2] as f64) * invdet) as f32;
        	result.m[4] = (((self.m[2] as f64) * (self.m[5] as f64) - (self.m[3] as f64) * (self.m[4] as f64)) * invdet) as f32;
        	result.m[1] = (-(self.m[1] as f64) * invdet) as f32;
        	result.m[3] = ((self.m[0] as f64) * invdet) as f32;
        	result.m[5] = (((self.m[1] as f64) * (self.m[4] as f64) - (self.m[0] as f64) * (self.m[5] as f64)) * invdet) as f32;
            Some(result)
        }
    }

    /// Transform a point by given transform.
    pub fn point(&self, sx: f32, sy: f32) -> (f32, f32) {
        let dx = sx * self.m[0] + sy * self.m[2] + self.m[4];
    	let dy = sx * self.m[1] + sy * self.m[3] + self.m[5];
        (dx, dy)
    }
}
