pub struct Vec3 {
    pub v: [f32; 3],
}

impl Vec3 {

	/// Constructor
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { v: [x, y, z] }
    }

	/// Get inner array
	pub fn get(&self) -> &[f32; 3] {
		&self.v
	}

	/// Get the value of x component.
	///
	/// # Examples
	///
	/// ```
	/// use 3D:vectors::Vec3;
    pub fn x(&self) -> f32 {
        self.v[0]
    }

    pub fn y(&self) -> f32 {
        self.v[1]
    }

    pub fn z(&self) -> f32 {
        self.v[2]
    }

    pub fn dot(&self) -> f32 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                self.v[2] * other.v[0] - self.v[0] * other.v[2],
                self.v[0] * other.v[1] - self.v[1] * other.v[0],
            ],
        }
    }

    pub fn mul(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2],
            ],
        }
    }

    pub fn div(&self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] / other.v[0],
                self.v[1] / other.v[1],
                self.v[2] / other.v[2],
            ],
        }
    }

    pub fn scale(&self, s: f32) -> Vec3 {
        Vec3 {
            v: [self.v[0] * s, self.v[1] * s, self.v[2] * s],
        }
    }
}

impl core::fmt::Display for Vec3 {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "[{:.4}, {:.4}, {:.4}]", self.v[0], self.v[1], self.v[2])
	}
}