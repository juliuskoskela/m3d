//! # Vector3
//!
//! A 3D vector.

use num::Float;

pub struct Vector3<F: Float> {
    pub v: [F; 3],
}

impl<F: Float> Vector3<F> {

	/// Constructor
    pub fn new(x: F, y: F, z: F) -> Vector3<F> {
        Vector3 { v: [x, y, z] }
    }

	/// Get inner array
	pub fn decompose(&self) -> [F; 3] {
		self.v
	}

	/// Get the value of x component.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.x(), 1.0);
	/// ```

    pub fn x(&self) -> F {
        self.v[0]
    }

	/// Get the value of y component.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.y(), 2.0);
	/// ```

    pub fn y(&self) -> F {
        self.v[1]
    }

	/// Get the value of z component.
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.z(), 3.0);
	/// ```

    pub fn z(&self) -> F {
        self.v[2]
    }

	/// Dot product of a vector is defined as the sum of the products of the corresponding components:
	///
	/// $$\vec{a} \cdot \vec{b} = a_x \cdot b_x + a_y \cdot b_y + a_z \cdot b_z$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vector::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert_eq!(v1.dot(), 14.0);
	/// ```

    pub fn dot(&self) -> F {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

	/// The cross product of two vectors is defined as:
	///
	/// $$\vec{a} \times \vec{b} = \left(\begin{array}{c} a_y \cdot b_z - a_z \cdot b_y \\ a_z \cdot b_x - a_x \cdot b_z \\ a_x \cdot b_y - a_y \cdot b_x \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vector::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert_eq!(v1.cross(&v2), Vector3::new(-3.0, 6.0, -3.0));
	/// ```

    pub fn cross(&self, other: Vector3<F>) -> Vector3<F> {
        Vector3 {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                self.v[2] * other.v[0] - self.v[0] * other.v[2],
                self.v[0] * other.v[1] - self.v[1] * other.v[0],
            ],
        }
    }

	/// The product of two vectors is defined as:
	///
	/// $$\vec{a} \times \vec{b} = \left(\begin{array}{c} a_x \cdot b_x + a_y \cdot b_y + a_z \cdot b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert!(v1 * v2 == Vector3::new(4.0, 10.0, 18.0));
	/// ```

    pub fn product(&self, other: Vector3<F>) -> Vector3<F> {
        Vector3 {
            v: [
                self.v[0] * other.v[0],
                self.v[1] * other.v[1],
                self.v[2] * other.v[2],
            ],
        }
    }

	/// The quotient of two vectors is defined as:
	///
	/// $$\vec{a} \div \vec{b} = \left(\begin{array}{c} a_x \div b_x + a_y \div b_y + a_z \div b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert!(v1 / v2 == Vector3::new(0.25, 0.4, 0.5));
	/// ```

    pub fn quotient(&self, other: Vector3<F>) -> Vector3<F> {
        Vector3 {
            v: [
                self.v[0] / other.v[0],
                self.v[1] / other.v[1],
                self.v[2] / other.v[2],
            ],
        }
    }

	/// Scalar multiplication of a vector is defined as:
	///
	/// $$\vec{a} \times c = \left(\begin{array}{c} a_x \times c \\ a_y \times c \\ a_z \times c \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.scalar_mul(2.0) == Vector3::new(2.0, 4.0, 6.0));
	/// ```

    pub fn scalar_mul(&self, s: F) -> Vector3<F> {
        Vector3 {
            v: [self.v[0] * s, self.v[1] * s, self.v[2] * s],
        }
    }

	/// Sum of two vectors is defined as:
	///
	/// $$\vec{a} + \vec{b} = \left(\begin{array}{c} a_x + b_x \\ a_y + b_y \\ a_z + b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert!(v1 + v2 == Vector3::new(5.0, 7.0, 9.0));
	/// ```

	pub fn sum(&self, other: Vector3<F>) -> Vector3<F> {
		Vector3 {
			v: [self.v[0] + other.v[0], self.v[1] + other.v[1], self.v[2] + other.v[2]],
		}
	}

	/// Subtraction of two vectors is defined as:
	///
	/// $$\vec{a} - \vec{b} = \left(\begin{array}{c} a_x - b_x \\ a_y - b_y \\ a_z - b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert!(v1 - v2 == Vector3::new(-3.0, -3.0, -3.0));
	/// ```

	pub fn difference(&self, other: Vector3<F>) -> Vector3<F> {
		Vector3 {
			v: [self.v[0] - other.v[0], self.v[1] - other.v[1], self.v[2] - other.v[2]],
		}
	}

	/// The magnitude of a vector is defined as:
	///
	/// $$\sqrt{a_x^2 + a_y^2 + a_z^2}$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.magnitude() == (1.0 as f64).sqrt() + (2.0 as f64).sqrt() + (3.0 as f64).sqrt());
	/// ```

	pub fn magnitude(&self) -> F {
		(self.dot()).sqrt()
	}

	/// The normalized vector is defined as:
	///
	/// $$\frac{\vec{a}}{\sqrt{a_x^2 + a_y^2 + a_z^2}}$$
	///
	/// # Examples
	///
	/// ```
	/// use math3D::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.normalized() == Vector3::new(1.0 / (1.0 as f64).sqrt(), 2.0 / (2.0 as f64).sqrt(), 3.0 / (3.0 as f64).sqrt()));
	/// ```

	pub fn normalized(&self) -> Vector3<F> {
		let mag = self.magnitude();
		Vector3 {
			v: [self.v[0] / mag, self.v[1] / mag, self.v[2] / mag],
		}
	}
}

impl<F: Float> core::fmt::Display for Vector3<F> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "[{:.4}, {:.4}, {:.4}]", self.v[0].to_f64().unwrap(), self.v[1].to_f64().unwrap(), self.v[2].to_f64().unwrap())
	}
}

impl<F: Float> core::cmp::PartialEq for Vector3<F> {
	fn eq(&self, other: &Vector3<F>) -> bool {
		self.v[0] == other.v[0] && self.v[1] == other.v[1] && self.v[2] == other.v[2]
	}
}

impl<F: Float> std::ops::Add for Vector3<F> {
	type Output = Vector3<F>;

	fn add(self, other: Vector3<F>) -> Vector3<F> {
		self.sum(other)
	}
}

impl<F: Float> std::ops::Sub for Vector3<F> {
	type Output = Vector3<F>;

	fn sub(self, other: Vector3<F>) -> Vector3<F> {
		self.difference(other)
	}
}

impl<F: Float> std::ops::Mul for Vector3<F> {
	type Output = Vector3<F>;

	fn mul(self, other: Vector3<F>) -> Vector3<F> {
		self.product(other)
	}
}

impl<F: Float> std::ops::Mul<F> for Vector3<F> {
	type Output = Vector3<F>;

	fn mul(self, other: F) -> Vector3<F> {
		self.scalar_mul(other)
	}
}

impl<F: Float> std::ops::Div for Vector3<F> {
	type Output = Vector3<F>;

	fn div(self, other: Vector3<F>) -> Vector3<F> {
		self.quotient(other)
	}
}