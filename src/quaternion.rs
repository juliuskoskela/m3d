//! # Quaternion
//!
//! Quaternion is a 4-dimensional vector.
//! It is used to represent rotations and rotational motion.
//!
//! # Example
//!
//! ```
//! use math3D::quaternion::Quaternion;
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

/// Structure representing a quaternion.
///
/// # Example
///
/// ```
/// use math3D::quaternion::Quaternion;
///
/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
/// ```

#[derive(Debug, Copy, Clone)]
pub struct Quaternion<F: Float>
{

	/// Real part of the quaternion.
	w: F,

	/// Scalar i of the quaternion.
	x: F,

	/// Scalar j of the quaternion.
	y: F,

	/// Scalar k of the quaternion.
	z: F,
}

impl<F: Float> Quaternion<F> {

	/// Creates a new quaternion from the given components.
	///
	/// # Arguments
	///
	/// * `w` - The real component of the quaternion.
	/// * `x` - The i component of the quaternion.
	/// * `y` - The j component of the quaternion.
	/// * `z` - The k component of the quaternion.
	///
	/// # Example
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	/// ```

	pub fn new(real: F, scalar: [F; 3]) -> Quaternion<F> {
		Quaternion {
			w: real,
			x: scalar[0],
			y: scalar[1],
			z: scalar[2],
		}
	}

	/// Get component of the quaternion.
	///
	/// # Example
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	///
	/// let w = q.get();
	/// ```

	pub fn get(&self) -> (F, [F; 3]) {
		(self.w, [self.x, self.y, self.z])
	}

	/// Create an identity quaternion.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
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
			w: F::from(1.0).unwrap(),
			x: F::from(0.0).unwrap(),
			y: F::from(0.0).unwrap(),
			z: F::from(0.0).unwrap(),
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
	/// use math3D::quaternion::Quaternion;
	///
	/// let q = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// ```

	pub fn from_axis_angle(axis: [F; 3], angle: F) -> Quaternion<F> {
		let half_angle = angle / F::from(2.0).unwrap();
		let sin_half_angle = (half_angle).sin();
		Quaternion {
			w: (half_angle).cos(),
			x: axis[0] * sin_half_angle,
			y: axis[1] * sin_half_angle,
			z: axis[2] * sin_half_angle,
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
	/// use math3D::quaternion::Quaternion;
	///
	/// let q = Quaternion::from_euler_angles(90.0, 0.0, 0.0);
	/// ```

	pub fn from_euler_angles(x: F, y: F, z: F) -> Quaternion<F> {
		let half_x = x / F::from(2.0).unwrap();
		let half_y = y / F::from(2.0).unwrap();
		let half_z = z / F::from(2.0).unwrap();
		let sin_half_x = half_x.sin();
		let sin_half_y = half_y.sin();
		let sin_half_z = half_z.sin();
		let cos_half_x = half_x.cos();
		let cos_half_y = half_y.cos();
		let cos_half_z = half_z.cos();
		Quaternion {
			w: cos_half_x * cos_half_y * cos_half_z + sin_half_x * sin_half_y * sin_half_z,
			x: sin_half_x * cos_half_y * cos_half_z - cos_half_x * sin_half_y * sin_half_z,
			y: cos_half_x * sin_half_y * cos_half_z + sin_half_x * cos_half_y * sin_half_z,
			z: cos_half_x * cos_half_y * sin_half_z - sin_half_x * sin_half_y * cos_half_z,
		}
	}

	/// Quarternion addition is defined as:
	///
	/// $$q_1 + q_2 = \frac{q_1 \cdot q_2}{|q_1|^2} + \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
	///
	/// # Arguments
	///
	/// * `other` - The quaternion to add.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
	/// let q3 = q1 + q2;
	/// ```

	pub fn sum(self, other: Quaternion<F>) -> Quaternion<F> {
		Quaternion {
			w: self.w + other.w,
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}

	/// Quarternion subtraction is defined as:
	///
	/// $$q_1 - q_2 = \frac{q_1 \cdot q_2}{|q_1|^2} - \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
	///
	/// # Arguments
	///
	/// * `other` - The quaternion to subtract.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
	/// let q3 = q1 - q2;
	/// ```

	pub fn difference(self, other: Quaternion<F>) -> Quaternion<F> {
		Quaternion {
			w: self.w - other.w,
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}

	/// Quarternion conjugate is defined as:
	///
	/// $$q_1^* = \frac{q_1}{|q_1|} + \frac{-i}{|q_1|} $$
	///
	/// Conjugate is used to invert a quaternion (rotate it the other way around).
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = q1.conjugate();
	/// ```


	pub fn conjugate(&self) -> Quaternion<F> {
		Quaternion {
			w: self.w,
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}

	/// Quarternion multiplication is defined as:
	///
	/// $$q_1 \cdot q_2 = \frac{q_1 \cdot q_2}{|q_1|^2} + \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
	///
	/// # Arguments
	///
	/// * `other` - The quaternion to multiply by.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
	/// let q3 = q1 * q2;
	/// ```

	pub fn product(self, other: Quaternion<F>) -> Quaternion<F> {
		Quaternion {
			w: -self.x * other.x - self.y * other.y - self.z * other.z + self.w * other.w,
			x:  self.x * other.w + self.y * other.z - self.z * other.y + self.w * other.x,
			y: -self.x * other.z + self.y * other.w + self.z * other.x + self.w * other.y,
			z:  self.x * other.y - self.y * other.x + self.z * other.w + self.w * other.z,
		}
	}

	/// Quarternion division is defined as:
	///
	/// $$q_1 / q_2 = \frac{q_1 \cdot q_2}{|q_1|^2} - \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
	///
	/// # Arguments
	///
	/// * `other` - The quaternion to divide by.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
	/// let q3 = q1 / q2;
	/// ```

	pub fn quotient(self, other: Quaternion<F>) -> Quaternion<F> {
		self * other.inverse()
	}

	/// Quarternion norm is defined as:
	///
	/// $$|q_1| = \sqrt{q_1 \cdot q_1} $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let norm = q1.norm();
	/// ```

	pub fn norm(&self) -> F {
		(self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	/// Quarternion normalization is defined as:
	///
	/// $$q_1 \cdot q_1 = \frac{q_1 \cdot q_1}{|q_1|^2} + \frac{q_2 \cdot q_1}{|q_2|^2} + \frac{i}{2} $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let norm = q1.normalize();
	/// ```

	pub fn normalize(&self) -> Quaternion<F> {
		let n = self.norm();
		Quaternion {
			w: self.w / n,
			x: self.x / n,
			y: self.y / n,
			z: self.z / n,
		}
	}

	/// Quarternion inverse is defined as:
	///
	/// $$q_1^* = \frac{q_1}{|q_1|} + \frac{-i}{|q_1|} $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
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
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let exp = q1.exp();
	/// ```

	pub fn exp(&self) -> Quaternion<F> {
		let n = self.norm();
		let c = n.cos();
		let s = n.sin();
		let q = self.normalize();
		Quaternion {
			w: c,
			x: s * q.x,
			y: s * q.y,
			z: s * q.z,
		}
	}

	/// Quarternion logarithm is defined as:
	///
	/// $$q_1 = \log(q_1) = \frac{\theta}{|q_1|} + \frac{i \theta}{|q_1|} $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// let log = q1.log();
	/// ```

	pub fn log(&self) -> Quaternion<F> {
		let n = self.norm();
		let c = n.ln();
		let q = self.normalize();
		Quaternion {
			w: c,
			x: q.x / n,
			y: q.y / n,
			z: q.z / n,
		}
	}

	/// Quarternion power is defined as:
	///
	/// $$q_1^n = \exp(n \log(q_1)) $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	///
	/// // let q2 = q1.pow(2.0);
	/// ```

	pub fn pow(self, n: F) -> Quaternion<F> {
		self.exp() * self.pow(n - F::from(1.0).unwrap())
	}

	/// Quarternion rotation is defined as:
	///
	/// $$q_1 \cdot q_2 = \cos(\theta) + \frac{i \sin(\theta)}{|q_1|} $$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::quaternion::Quaternion;
	///
	/// let q1 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], 90.0);
	/// let q2 = Quaternion::from_axis_angle([0.0, 1.0, 0.0], 90.0);
	/// let rotated = q1.rotate(q2);
	/// ```

	pub fn rotate(&self, other: Quaternion<F>) -> Quaternion<F> {
		self.exp() * other * self.inverse()
	}

	/// Quarternion rotation is defined as:
	///
	/// out = self * other * self.conjugate()
	pub fn rotate_vector(&self, other: Vector3<F>) -> Vector3<F> {
		let q = Quaternion {
			w: F::from(0.0).unwrap(),
			x: other.x(),
			y: other.y(),
			z: other.z(),
		};
		let rotated = self.rotate(q);
		let (_, [x, y, z]) = rotated.get();
		Vector3::new(x, y, z)
	}
}

impl<F: Float> core::fmt::Display for Quaternion<F> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "( w: {}, x: {}, y: {}, z: {} )", self.w.to_f64().unwrap(), self.x.to_f64().unwrap(), self.y.to_f64().unwrap(), self.z.to_f64().unwrap())
	}
}

impl<F: Float> std::cmp::PartialEq for Quaternion<F> {
	fn eq(&self, other: &Quaternion<F>) -> bool {
		self.w == other.w && self.x == other.x && self.y == other.y && self.z == other.z
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
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
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
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}