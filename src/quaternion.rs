//! # Quaternion
//!
//! Quaternion is a 4-dimensional vector.
//! It is used to represent rotations and rotational motion.
//!
//! # Example
//!
//! ```
//! use m3d::quaternion::Quaternion;
//!
//! let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
//! let q2 = Quaternion::new(5.0, [6.0, 7.0, 8.0]);
//!
//! let q3 = q1 * q2;
//!
//! println!("{}", q3);
//! ```

use num::Float;

use crate::vectors::Vector3;
use crate::matrices::Matrix3;
use crate::matrices::Matrix4;

/// Structure representing a quaternion.
///
/// # Example
///
/// ```
/// use m3d::quaternion::Quaternion;
///
/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
/// ```

#[derive(Debug, Copy, Clone)]
pub struct Quaternion<F: Float> {
    /// Real part of the quaternion.
    w: F,

    /// Vector part of the quaternion.
    v: Vector3<F>,
}

impl<F: Float> Quaternion<F> {
    /// Creates a new quaternion from the given components.
    ///
    /// # Arguments
    ///
	/// * `w` - Real part of the quaternion.
	/// * `v` - Vector part of the quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
    /// ```

    pub fn new(w: F, v: [F; 3]) -> Quaternion<F> {
		Quaternion {
			w,
			v: Vector3::new(v[0], v[1], v[2]),
		}
    }

	/// Get the real part of the quaternion.
	///
	/// # Example
	///
	/// ```
	/// use m3d::quaternion::Quaternion;
	///
	/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	///
	/// assert_eq!(q.w(), 1.0);
	/// ```

	pub fn real(&self) -> F {
		self.w
	}

	/// Get the vector part of the quaternion.
	///
	/// # Example
	///
	/// ```
	/// use m3d::quaternion::Quaternion;
	///
	/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	///
	/// assert_eq!(q.v(), [2.0, 3.0, 4.0]);
	/// ```

	pub fn vector(&self) -> Vector3<F> {
		self.v
	}

	/// Quaternion as a vetor and a scalar.
	///
	/// # Example
	///
	/// ```
	/// use m3d::quaternion::Quaternion;
	/// use m3d::vectors::Vector3;
	///
	/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	///
	/// let (w, v) = q.vector_and_scalar();
	///
	/// assert_eq!(w, 1.0);
	/// assert!(v == Vector3::new(2.0, 3.0, 4.0));
	/// ```

	pub fn vector_and_scalar(&self) -> (F, Vector3<F>) {
		(self.w, self.v)
	}

    /// Get component of the quaternion as a tuple of a scalar and a vector.
    ///
    /// # Example
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	///
	/// let (w, x, y, z) = q.decompose();
	///
	/// assert_eq!(w, 1.0);
	/// assert_eq!(x, 2.0);
	/// assert_eq!(y, 3.0);
	/// assert_eq!(z, 4.0);
    /// ```

    pub fn decompose(&self) -> (F, F, F, F) {
		(self.w, self.v[0], self.v[1], self.v[2])
    }

    /// Create an identity quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q = Quaternion::<f32>::identity();
    /// let (r, [x, y, z]) = q.get();
    /// assert_eq!(r, 1.0);
    /// assert_eq!(x, 0.0);
    /// assert_eq!(y, 0.0);
    /// assert_eq!(z, 0.0);
    /// ```

    pub fn identity() -> Quaternion<F> {
        Quaternion {
            w: F::one(),
			v: Vector3::zero(),
        }
    }

    /// From the given axis and angle, create a quaternion.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis of rotation.
    /// * `angle` - The angle of rotation.
    ///
    /// # Example
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// ```

    pub fn from_axis_angle(axis: Vector3<F>, angle: F) -> Quaternion<F> {
        let half_angle = angle.to_radians() / F::from(2.0).unwrap();

        Quaternion {
            w: half_angle.cos(),
            v: axis * half_angle.sin(),
        }
    }

    /// From the given euler angles, create a quaternion.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-axis euler angle.
    /// * `y` - The y-axis euler angle.
    /// * `z` - The z-axis euler angle.
    ///
    /// # Example
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q = Quaternion::from_euler_angles(90.0, 0.0, 0.0);
    /// ```

    pub fn from_euler_angles(x: F, y: F, z: F) -> Quaternion<F> {
        let half_x = x.to_radians() / F::from(2.0).unwrap();
        let half_y = y.to_radians() / F::from(2.0).unwrap();
        let half_z = z.to_radians() / F::from(2.0).unwrap();

        let sin_x = half_x.sin();
        let sin_y = half_y.sin();
        let sin_z = half_z.sin();

        let cos_x = half_x.cos();
        let cos_y = half_y.cos();
        let cos_z = half_z.cos();

        Quaternion {
            w: cos_x * cos_y * cos_z + sin_x * sin_y * sin_z,
			v: Vector3::new(
				cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
				cos_x * cos_y * sin_z - sin_x * sin_y * cos_z,
				cos_x * sin_y * sin_z + sin_x * cos_y * cos_z,
			),
        }
    }

    /// The sum of two quaternions:
    ///
    /// $$ q = q1 + q2 $$
    ///
    /// is defined as:
    ///
    /// $$ q = (w1 + w2, x1 + x2, y1 + y2, z1 + z2) $$
    ///
    /// # Arguments
    ///
    /// * `other` - The quaternion to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
    /// let q3 = q1 + q2;
    /// ```

    pub fn sum(self, other: Quaternion<F>) -> Quaternion<F> {
        Quaternion {
            w: self.w + other.w,
            v: self.v + other.v,
        }
    }

    /// The difference of two quaternions:
    ///
    /// $$ q = q1 - q2 $$
    ///
    /// is defined as:
    ///
    /// $$ q = (w1 - w2, x1 - x2, y1 - y2, z1 - z2) $$
    ///
    /// # Arguments
    ///
    /// * `other` - The quaternion to subtract.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
    /// let q3 = q1 - q2;
    /// ```

    pub fn difference(self, other: Quaternion<F>) -> Quaternion<F> {
        Quaternion {
            w: self.w - other.w,
            v: self.v - other.v,
        }
    }

    /// The conjugate of a quaternion:
    ///
    /// $$ q = conj(q1) $$
    ///
    /// is defined as:
    ///
    /// $$ q = (w1, -x1, -y1, -z1) $$
    ///
    /// Conjugate is used to invert a quaternion (rotate it the other way around).
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// let q2 = q1.conjugate();
    /// ```

    pub fn conjugate(&self) -> Quaternion<F> {
        Quaternion {
            w: self.w,
            v: -self.v,
        }
    }

    /// The product of two quaternions:
    ///
    /// $$ q = q1 * q2 $$
    ///
    /// is defined as:
    ///
    /// [q1w, [q1v]] * [q2w, [q2v]] = [q1w * q2w − [q1v] ⋅ [q2v], q1w * [q2v] + q2w * [q1v] + [q1v] × [q2v]]
    ///
    /// # Arguments
    ///
    /// * `other` - The quaternion to multiply by.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
    /// let q3 = q1 * q2;
    /// ```

    pub fn product(self, other: Quaternion<F>) -> Quaternion<F> {
		let (w1, v1) = self.vector_and_scalar();
		let (w2, v2) = other.vector_and_scalar();

		Quaternion {
			w: w1 * w2 - v1.dot(v2),
			v: v1.cross(v2) + v1 * w2 + v2 * w1,
		}
    }

    /// The quotient of two quaternions:
    ///
    /// $$ q = q1 / q2 $$
    ///
    /// is defined as:
    ///
    /// $$ q = q1 * conj(q2) / |q2|^2 $$
    ///
    /// # Arguments
    ///
    /// * `other` - The quaternion to divide by.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    /// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
    /// let q3 = q1 / q2;
    /// ```

    pub fn quotient(self, other: Quaternion<F>) -> Quaternion<F> {

		let (q1w, q1x, q1y, q1z) = self.decompose();
		let (q2w, q2x, q2y, q2z) = other.decompose();

        // q.w = ( q1.w * q2.w + q1.x * q2.x + q1.y * q2.y + q1.z * q2.z ) / ( q1.w^2 + q1.x^2 + q1.y^2 + q1.z^2 )
        // q.x = ( q1.w * q2.x - q1.x * q2.w + q1.y * q2.z - q1.z * q2.y ) / ( q1.w^2 - q1.x^2 + q1.y^2 - q1.z^2 )
        // q.y = ( q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x ) / ( q1.w^2 - q1.x^2 - q1.y^2 + q1.z^2 )
        // q.z = ( q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w ) / ( q1.w^2 + q1.x^2 - q1.y^2 - q1.z^2 )

		Quaternion {
			w: (q1w * q2w + q1x * q2x + q1y * q2y + q1z * q2z) / (q1w * q1w + q1x * q1x + q1y * q1y + q1z * q1z),
			v: Vector3::new(
				(q1w * q2x - q1x * q2w + q1y * q2z - q1z * q2y) / (q1w * q1w - q1x * q1x + q1y * q1y - q1z * q1z),
				(q1w * q2y - q1x * q2z + q1y * q2w + q1z * q2x) / (q1w * q1w - q1x * q1x - q1y * q1y + q1z * q1z),
				(q1w * q2z + q1x * q2y - q1y * q2x + q1z * q2w) / (q1w * q1w + q1x * q1x - q1y * q1y - q1z * q1z),
			),
		}
    }

    /// Quarternion norm is defined as:
    ///
    /// $$|q_1| = \sqrt{q_1 \cdot q_1} $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// let norm = q1.norm();
    /// ```

    pub fn norm(&self) -> F {
        (self.w * self.w + self.v.dot(self.v)).sqrt()
    }

    /// Quarternion versor is defined as:
    ///
    /// $$q_1 \cdot q_1 = \frac{q_1 \cdot q_1}{|q_1|^2} + \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// let norm = q1.versor();
    /// ```

    pub fn versor(&self) -> Quaternion<F> {
        let n = self.norm();
        Quaternion {
            w: self.w / n,
            v: self.v / n,
        }
    }

    /// Quarternion inverse is defined as:
    ///
    /// $$q_1^* = \frac{q_1}{|q_1|} + \frac{-i}{|q_1|} $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// let inverse = q1.inverse();
    /// ```

    pub fn inverse(&self) -> Quaternion<F> {
        self.conjugate() / (self.norm() * self.norm())
    }

    /// Quarternion exponential is defined as:
    ///
    /// $$q_1 = \exp(q_1) = \cos(\theta) + \frac{i \sin(\theta)}{|q_1|} $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// let exp = q1.exp();
    /// ```

    pub fn exp(&self) -> Quaternion<F> {
        let n = self.norm();
        let c = n.cos();
        let s = n.sin();
        let q = self.versor();
		Quaternion {
			w: c,
			v: q.v * s,
		}
    }

    /// Quarternion logarithm is defined as:
    ///
    /// $$q_1 = \log(q_1) = \frac{\theta}{|q_1|} + \frac{i \theta}{|q_1|} $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// let log = q1.log();
    /// ```

    pub fn log(&self) -> Quaternion<F> {
        let n = self.norm();
        let c = n.ln();
        let q = self.versor();
		Quaternion {
			w: c,
			v: q.v * c,
		}
    }

    /// Quarternion power is defined as:
    ///
    /// $$q_1^n = \exp(n \log(q_1)) $$
    ///
    /// # Examples
    ///
    /// ```
    /// use m3d::quaternion::Quaternion;
    ///
    /// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
    ///
    /// // let q2 = q1.pow(2.0);
    /// ```

    pub fn pow(self, n: F) -> Quaternion<F> {
        self.exp() * self.pow(n - F::from(1.0).unwrap())
    }

	/// Rotating a vector by a quaternion is defined as:
	///
	/// $$v_1 = q_1 \cdot v_1 \cdot q_1^* $$
	///
	/// # Examples
	///
	/// ```
	/// use m3d::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let v1 = [1.0, 0.0, 0.0];
	///
	/// let v2 = q1.rotate(v1);
	/// ```

	pub fn rotate_vector(&self, v: Vector3<F>) -> Vector3<F> {
		let p_in = Quaternion {
			w: F::from(0.0).unwrap(),
			v: v,
		};
		(*self * p_in * self.conjugate()).v
	}

	/// Quaternion rotation to Matrix3
	///
	/// (2w^2 − 1 + 2x^2) (2xy + 2wz) (2xz − 2wy)
	/// (2xy − 2wz) (2w^2 − 1 + 2y^2) (2yz + 2wx)
	/// (2xz + 2wy) (2yz − 2wx) (2w^2 − 1 + 2z^2)
	///
	/// # Examples
	///
	/// ```
	/// use m3d::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let m1 = q1.to_matrix();
	/// ```

	pub fn rotation_matrix(&self) -> Matrix3<F> {
		let two = F::from(2.0).unwrap();
		let mut m = Matrix3::identity();
		m[0][0] = two * self.w * self.w - F::one() + two * self.v[0] * self.v[0];
		m[0][1] = two * self.v[0] * self.v[1] + two * self.w * self.v[2];
		m[0][2] = two * self.v[0] * self.v[2] - two * self.w * self.v[1];
		m[1][0] = two * self.v[0] * self.v[1] - two * self.w * self.v[2];
		m[1][1] = two * self.w * self.w - F::one() + two * self.v[1] * self.v[1];
		m[1][2] = two * self.v[1] * self.v[2] + two * self.w * self.v[0];
		m[2][0] = two * self.v[0] * self.v[2] + two * self.w * self.v[1];
		m[2][1] = two * self.v[1] * self.v[2] - two * self.w * self.v[0];
		m[2][2] = two * self.w * self.w - F::one() + two * self.v[2] * self.v[2];
		m
	}
}

impl<F: Float> core::fmt::Display for Quaternion<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "(s: {}, i: {}, j: {}, k: {})",
            self.w.to_f64().unwrap(),
            self.v.x().to_f64().unwrap(),
            self.v.y().to_f64().unwrap(),
            self.v.z().to_f64().unwrap()
        )
    }
}

impl<F: Float> std::cmp::PartialEq for Quaternion<F> {
    fn eq(&self, other: &Quaternion<F>) -> bool {
        self.w == other.w && self.v == other.v
    }
}

impl<F: Float> std::ops::Add for Quaternion<F> {
    type Output = Quaternion<F>;

    fn add(self, other: Quaternion<F>) -> Quaternion<F> {
        self.sum(other)
    }
}

impl<F: Float> std::ops::Sub for Quaternion<F> {
    type Output = Quaternion<F>;

    fn sub(self, other: Quaternion<F>) -> Quaternion<F> {
        self.difference(other)
    }
}

impl<F: Float> std::ops::Mul for Quaternion<F> {
    type Output = Quaternion<F>;

    fn mul(self, other: Quaternion<F>) -> Quaternion<F> {
        self.product(other)
    }
}

impl<F: Float> std::ops::Mul<F> for Quaternion<F> {
    type Output = Quaternion<F>;

    fn mul(self, other: F) -> Quaternion<F> {
        Quaternion {
            w: self.w * other,
            v: self.v * other,
        }
    }
}

impl<F: Float> std::ops::Div for Quaternion<F> {
    type Output = Quaternion<F>;

    fn div(self, other: Quaternion<F>) -> Quaternion<F> {
        self.quotient(other)
    }
}

impl<F: Float> std::ops::Div<F> for Quaternion<F> {
    type Output = Quaternion<F>;

    fn div(self, other: F) -> Quaternion<F> {
        Quaternion {
            w: self.w / other,
            v: self.v / other,
        }
    }
}

impl<F: Float> std::ops::Index<usize> for Quaternion<F> {
	type Output = F;

	fn index(&self, index: usize) -> &F {
		match index {
			0 => &self.w,
			1 => &self.v[0],
			2 => &self.v[1],
			3 => &self.v[2],
			_ => panic!("Index out of bounds"),
		}
	}
}

impl<F: Float> std::ops::IndexMut<usize> for Quaternion<F> {
	fn index_mut(&mut self, index: usize) -> &mut F {
		match index {
			0 => &mut self.w,
			1 => &mut self.v[0],
			2 => &mut self.v[1],
			3 => &mut self.v[2],
			_ => panic!("Index out of bounds"),
		}
	}
}