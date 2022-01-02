//! # Vector
//!
//! Vector types Vector2, vector3 and Vector4

use num::Float;
use crate::matrices::Matrix3;

// //////////////////////////////////////////////////////////////////////////////////////
//
// Vector3
//
// //////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct Vector3<F: Float> {
    v: [F; 3],
}

impl<F: Float> Vector3<F> {

	/// Constructor for Vector3 from a list of 3 values.
    pub fn new(x: F, y: F, z: F) -> Vector3<F> {
        Vector3 { v: [x, y, z] }
    }

	/// Construct a zero vector.
	pub fn zero() -> Vector3<F> {
		Vector3 { v: [F::zero(), F::zero(), F::zero()] }
	}

	pub fn identity() -> Vector3<F> {
		Vector3 { v: [F::one(), F::zero(), F::zero()] }
	}

	/// As slice.
	pub fn as_slice(&self) -> &[F; 3] {
		&self.v
	}

    /// From array.
	///
	/// # Arguments
	///
	/// * `v` - The array to create the vector from.
	///
	/// # Example
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v = Vector3::from_array([1.0, 2.0, 3.0]);
	/// ```

	pub fn from_array(v: [F; 3]) -> Vector3<F> {
		Vector3 { v }
	}

	/// Decompose the vector into a tuple of 3 values.
	///
	/// # Example
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let vector = Vector3::new(1.0, 2.0, 3.0);
	/// let (x, y, z) = vector.decompose();
	///
	/// assert_eq!(x, 1.0);
	/// assert_eq!(y, 2.0);
	/// assert_eq!(z, 3.0);
	/// ```

	pub fn decompose(&self) -> (F, F, F) {
		(self.v[0], self.v[1], self.v[2])
	}

	/// Get the value of x component.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.x(), 1.0);
	/// ```

    pub fn x(&self) -> &F {
        &self.v[0]
    }

	/// Get the value of y component.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.y(), 2.0);
	/// ```

    pub fn y(&self) -> &F {
        &self.v[1]
    }

	/// Get the value of z component.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vector::Vector3;
	///
	/// let v = Vector3::new(1.0, 2.0, 3.0);
	/// assert_eq!(v.z(), 3.0);
	/// ```

    pub fn z(&self) -> &F {
        &self.v[2]
    }

	/// Sum of two vectors is defined as:
	///
	/// $$\vec{a} + \vec{b} = \left(\begin{array}{c} a_x + b_x \\ a_y + b_y \\ a_z + b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
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

	/// Scalar sum of a vector is defined as:
	///
	/// $$\vec{a} + c = \left(\begin{array}{c} a_x + c \\ a_y + c \\ a_z + c \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1 + 2.0 == Vector3::new(3.0, 4.0, 5.0));
	/// ```

	pub fn sum_scalar(&self, scalar: F) -> Vector3<F> {
		Vector3 {
			v: [self.v[0] + scalar, self.v[1] + scalar, self.v[2] + scalar],
		}
	}

	/// Subtraction of two vectors is defined as:
	///
	/// $$\vec{a} - \vec{b} = \left(\begin{array}{c} a_x - b_x \\ a_y - b_y \\ a_z - b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
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

	/// Scalar subtraction of a vector is defined as:
	///
	/// $$\vec{a} - c = \left(\begin{array}{c} a_x - c \\ a_y - c \\ a_z - c \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1 - 2.0 == Vector3::new(-1.0, 0.0, 1.0));
	/// ```

	pub fn difference_scalar(&self, scalar: F) -> Vector3<F> {
		Vector3 {
			v: [self.v[0] - scalar, self.v[1] - scalar, self.v[2] - scalar],
		}
	}

	/// The product of two vectors is defined as:
	///
	/// $$\vec{a} \times \vec{b} = \left(\begin{array}{c} a_x \cdot b_x + a_y \cdot b_y + a_z \cdot b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
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

	// Scalar multiplication of a vector is defined as:
	///
	/// $$\vec{a} \times c = \left(\begin{array}{c} a_x \times c \\ a_y \times c \\ a_z \times c \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.product_scalar(2.0) == Vector3::new(2.0, 4.0, 6.0));
	/// ```

    pub fn product_scalar(&self, s: F) -> Vector3<F> {
        Vector3 {
            v: [self.v[0] * s, self.v[1] * s, self.v[2] * s],
        }
    }

	/// Matrix multiplication of a vector is defined as:
	///
	/// $$\vec{a} \times \vec{b} = \left(\begin{array}{c} a_x \times b_x + a_y \times b_y + a_z \times b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	/// let v2 = Vector3::new(4.0, 5.0, 6.0);
	///
	/// assert!(v1.product_matrix(v2) == Vector3::new(4.0, 10.0, 18.0));
	/// ```

	pub fn product_matrix(&self, other: Matrix3<F>) -> Vector3<F> {
		let mut result = Vector3::<F>::zero();

		for i in 0..3 {
			for j in 0..3 {
				result[i] = result[i] + self[j] * other[j][i];
			}
		}
		result
	}

	/// The quotient of two vectors is defined as:
	///
	/// $$\vec{a} \div \vec{b} = \left(\begin{array}{c} a_x \div b_x + a_y \div b_y + a_z \div b_z \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
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

	/// The scalar quotient of two vectors is defined as:
	///
	/// $$\vec{a} \div c = \left(\begin{array}{c} a_x \div c \\ a_y \div c \\ a_z \div c \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.quotient_scalar(2.0) == Vector3::new(0.5, 1.0, 1.5));

	pub fn quotient_scalar(&self, s: F) -> Vector3<F> {
		Vector3 {
			v: [self.v[0] / s, self.v[1] / s, self.v[2] / s],
		}
	}

	/// Dot product of a vector is defined as the sum of the products of the corresponding components:
	///
	/// $$\vec{a} \cdot \vec{b} = a_x \cdot b_x + a_y \cdot b_y + a_z \cdot b_z$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vector::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert_eq!(v1.dot(), 14.0);
	/// ```

    pub fn dot(&self, other: Vector3<F>) -> F {
		self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

	/// The cross product of two vectors is defined as:
	///
	/// $$\vec{a} \times \vec{b} = \left(\begin{array}{c} a_y \cdot b_z - a_z \cdot b_y \\ a_z \cdot b_x - a_x \cdot b_z \\ a_x \cdot b_y - a_y \cdot b_x \end{array}\right)$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vector::Vector3;
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

	/// The magnitude of a vector is defined as:
	///
	/// $$\sqrt{a_x^2 + a_y^2 + a_z^2}$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 0.0, 0.0);
	///
	/// assert_eq!(v1.magnitude(), 1.0);
	/// ```

	pub fn magnitude(&self) -> F {
		(self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]).sqrt()
	}

	/// The normalized vector is defined as:
	///
	/// $$\frac{\vec{a}}{\sqrt{a_x^2 + a_y^2 + a_z^2}}$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
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

	/// The opposite vector is defined as:
	///
	/// $$\vec{a} \times -1$$
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector3;
	///
	/// let v1 = Vector3::new(1.0, 2.0, 3.0);
	///
	/// assert!(v1.opposite() == Vector3::new(-1.0, -2.0, -3.0));
	/// ```

	pub fn opposite(&self) -> Vector3<F> {
		Vector3 {
			v: [self.v[0].neg(), self.v[1].neg(), self.v[2].neg()],
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

impl<F: Float> std::ops::Add<F> for Vector3<F> {
	type Output = Vector3<F>;

	fn add(self, other: F) -> Vector3<F> {
		self.sum_scalar(other)
	}
}

impl<F: Float> std::ops::Sub for Vector3<F> {
	type Output = Vector3<F>;

	fn sub(self, other: Vector3<F>) -> Vector3<F> {
		self.difference(other)
	}
}

impl<F: Float> std::ops::Sub<F> for Vector3<F> {
	type Output = Vector3<F>;

	fn sub(self, other: F) -> Vector3<F> {
		self.difference_scalar(other)
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
		self.product_scalar(other)
	}
}

impl<F: Float> std::ops::Mul<Matrix3<F>> for Vector3<F> {
	type Output = Vector3<F>;

	fn mul(self, other: Matrix3<F>) -> Vector3<F> {
		self.product_matrix(other)
	}
}

impl<F: Float> std::ops::Div for Vector3<F> {
	type Output = Vector3<F>;

	fn div(self, other: Vector3<F>) -> Vector3<F> {
		self.quotient(other)
	}
}

impl<F: Float> std::ops::Div<F> for Vector3<F> {
	type Output = Vector3<F>;

	fn div(self, other: F) -> Vector3<F> {
		self.quotient_scalar(other)
	}
}

impl<F: Float> std::ops::Neg for Vector3<F> {
	type Output = Vector3<F>;

	fn neg(self) -> Vector3<F> {
		self.opposite()
	}
}

impl<F: Float> std::ops::Index<usize> for Vector3<F> {
	type Output = F;

	fn index(&self, index: usize) -> &F {
		&self.v[index]
	}
}

impl<F: Float> std::ops::IndexMut<usize> for Vector3<F> {
	fn index_mut(&mut self, index: usize) -> &mut F {
		&mut self.v[index]
	}
}

// //////////////////////////////////////////////////////////////////////////////////////
//
// Vector4
//
// //////////////////////////////////////////////////////////////////////////////////////

use crate::matrices::Matrix4;

#[derive(Clone, Copy, Debug)]
pub struct Vector4<F: Float> {
	v: [F; 4],
}

impl<F: Float> Vector4<F> {

	/// Creates a new Vector4 from the given components.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// assert!(v1 == Vector4::new(1.0, 2.0, 3.0, 4.0));
	/// ```

	pub fn new(x: F, y: F, z: F, w: F) -> Vector4<F> {
		Vector4 {
			v: [x, y, z, w],
		}
	}

	/// Creates a new Vector4 from the given Vector3 and w component.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new_from_vector3(Vector3::new(1.0, 2.0, 3.0), 4.0);
	///
	/// assert!(v1 == Vector4::new(1.0, 2.0, 3.0, 4.0));
	/// ```

	pub fn new_from_vector3(v: Vector3<F>, w: F) -> Vector4<F> {
		Vector4 {
			v: [v.v[0], v.v[1], v.v[2], w],
		}
	}

	/// Creates a zero Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::zero();
	///
	/// assert!(v1 == Vector4::new(0.0, 0.0, 0.0, 0.0));
	/// ```

	pub fn zero() -> Vector4<F> {
		Vector4 {
			v: [F::zero(), F::zero(), F::zero(), F::zero()],
		}
	}

	/// Creates a identity Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::identity();
	///
	/// assert!(v1 == Vector4::new(1.0, 1.0, 1.0, 1.0));
	/// ```

	pub fn identity() -> Vector4<F> {
		Vector4 {
			v: [F::one(), F::one(), F::one(), F::one()],
		}
	}

	/// Sums the given Vector4 to this Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	/// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
	///
	/// let v3 = v1.sum(v2);
	///
	/// assert!(v3 == Vector4::new(6.0, 8.0, 10.0, 12.0));
	/// ```

	pub fn sum(self, other: Vector4<F>) -> Vector4<F> {
		Vector4 {
			v: [self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]],
		}
	}

	/// Scalar sum of this Vector4 and the given scalar.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let v2 = v1.sum_scalar(5.0);
	///
	/// assert!(v2 == Vector4::new(6.0, 7.0, 8.0, 9.0));
	/// ```

	pub fn sum_scalar(self, scalar: F) -> Vector4<F> {
		Vector4 {
			v: [self[0] + scalar, self[1] + scalar, self[2] + scalar, self[3] + scalar],
		}
	}

	/// Subtracts the given Vector4 from this Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	/// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
	///
	/// let v3 = v1.difference(v2);
	/// assert!(v3 == Vector4::new(-4.0, -4.0, -4.0, -4.0));
	/// ```

	pub fn difference(self, other: Vector4<F>) -> Vector4<F> {
		Vector4 {
			v: [self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]],
		}
	}


	/// Scalar difference of this Vector4 and the given scalar.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let v2 = v1.difference_scalar(5.0);
	///
	/// assert!(v2 == Vector4::new(-4.0, -3.0, -2.0, -1.0));
	/// ```

	pub fn difference_scalar(self, scalar: F) -> Vector4<F> {
		Vector4 {
			v: [self[0] - scalar, self[1] - scalar, self[2] - scalar, self[3] - scalar],
		}
	}

	/// Multiplies the given Vector4 with this Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	/// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
	///
	/// let v3 = v1.product(v2);
	/// assert!(v3 == Vector4::new(5.0, 12.0, 21.0, 32.0));
	/// ```

	pub fn product(self, other: Vector4<F>) -> Vector4<F> {
		Vector4 {
			v: [self[0] * other[0], self[1] * other[1], self[2] * other[2], self[3] * other[3]],
		}
	}

	/// Scalar product of this Vector4 and the given scalar.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let v2 = v1.product_scalar(5.0);
	///
	/// assert!(v2 == Vector4::new(5.0, 10.0, 15.0, 20.0));
	/// ```

	pub fn product_scalar(self, scalar: F) -> Vector4<F> {
		Vector4 {
			v: [self[0] * scalar, self[1] * scalar, self[2] * scalar, self[3] * scalar],
		}
	}

	/// Multiply Vector4 with Matrix4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let m1 = math3d::matrices::Matrix4::identity();
	///
	/// let v2 = v1.product_matrix(m1);
	///
	/// assert!(v2 == Vector4::new(1.0, 2.0, 3.0, 4.0));
	/// ```

	pub fn product_matrix(self, matrix: Matrix4<F>) -> Vector4<F> {
		let mut result = Vector4::zero();

		for i in 0..4 {
			for j in 0..4 {
				result[i] = result[i] + self[j] * matrix[i][j];
			}
		}
		result
	}

	/// Quotient of this Vector4 and the given Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	/// let v2 = Vector4::new(2.0, 2.0, 3.0, 4.0);
	///
	/// let v3 = v1.quotient(v2);
	/// assert!(v3 == Vector4::new(0.5, 1.0, 1.5, 2.0));
	/// ```

	pub fn quotient(self, other: Vector4<F>) -> Vector4<F> {
		Vector4 {
			v: [self[0] / other[0], self[1] / other[1], self[2] / other[2], self[3] / other[3]],
		}
	}

	/// Scalar quotient of this Vector4 and the given scalar.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let v2 = v1.quotient_scalar(2.0);
	///
	/// assert!(v2 == Vector4::new(0.5, 1.0, 1.5, 2.0));
	/// ```

	pub fn quotient_scalar(self, scalar: F) -> Vector4<F> {
		Vector4 {
			v: [self[0] / scalar, self[1] / scalar, self[2] / scalar, self[3] / scalar],
		}
	}

	/// Returns the dot product of this Vector4 and the given Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let v2 = Vector4::new(2.0, 2.0, 3.0, 4.0);
	///
	/// let dot = v1.dot(v2);
	///
	/// assert!(dot == 40.0);

	pub fn dot(self, other: Vector4<F>) -> F {
		self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
	}

	/// Magnitude of this Vector4.
	///
	/// # Examples
	///
	/// ```
	/// use math3d::vectors::Vector4;
	///
	/// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
	///
	/// let mag = v1.magnitude();
	///
	/// assert!(mag == 5.477225575051661);
	/// ```

	pub fn magnitude(self) -> F {
		(self[0] * self[0] + self[1] * self[1] + self[2] * self[2] + self[3] * self[3]).sqrt()
	}
}

impl<F: Float> core::fmt::Display for Vector4<F> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "v: ({:.4}, {:.4}, {:.4}, {:.4})", self.v[0].to_f64().unwrap(), self.v[1].to_f64().unwrap(), self.v[2].to_f64().unwrap(), self.v[3].to_f64().unwrap())
	}
}

impl<F: Float> core::cmp::PartialEq for Vector4<F> {
	fn eq(&self, other: &Vector4<F>) -> bool {
		self.v[0] == other.v[0] && self.v[1] == other.v[1] && self.v[2] == other.v[2] && self.v[3] == other.v[3]
	}
}

impl<F: Float> std::ops::Add for Vector4<F> {
	type Output = Vector4<F>;

	fn add(self, other: Vector4<F>) -> Vector4<F> {
		self.sum(other)
	}
}

impl<F: Float> std::ops::Add<F> for Vector4<F> {
	type Output = Vector4<F>;

	fn add(self, other: F) -> Vector4<F> {
		self.sum_scalar(other)
	}
}

impl<F: Float> std::ops::Sub for Vector4<F> {
	type Output = Vector4<F>;

	fn sub(self, other: Vector4<F>) -> Vector4<F> {
		self.difference(other)
	}
}

impl<F: Float> std::ops::Sub<F> for Vector4<F> {
	type Output = Vector4<F>;

	fn sub(self, other: F) -> Vector4<F> {
		self.difference_scalar(other)
	}
}

impl<F: Float> std::ops::Mul for Vector4<F> {
	type Output = Vector4<F>;

	fn mul(self, other: Vector4<F>) -> Vector4<F> {
		self.product(other)
	}
}

impl<F: Float> std::ops::Mul<F> for Vector4<F> {
	type Output = Vector4<F>;

	fn mul(self, other: F) -> Vector4<F> {
		self.product_scalar(other)
	}
}

impl<F: Float> std::ops::Mul<Matrix4<F>> for Vector4<F> {
	type Output = Vector4<F>;

	fn mul(self, other: Matrix4<F>) -> Vector4<F> {
		self.product_matrix(other)
	}
}

impl<F: Float> std::ops::Div for Vector4<F> {
	type Output = Vector4<F>;

	fn div(self, other: Vector4<F>) -> Vector4<F> {
		self.quotient(other)
	}
}

impl<F: Float> std::ops::Div<F> for Vector4<F> {
	type Output = Vector4<F>;

	fn div(self, other: F) -> Vector4<F> {
		self.quotient_scalar(other)
	}
}

impl<F: Float> std::ops::Neg for Vector4<F> {
	type Output = Vector4<F>;

	fn neg(self) -> Vector4<F> {
		todo!()
	}
}

impl<F: Float> std::ops::Index<usize> for Vector4<F> {
	type Output = F;

	fn index(&self, index: usize) -> &F {
		&self.v[index]
	}
}

impl<F: Float> std::ops::IndexMut<usize> for Vector4<F> {
	fn index_mut(&mut self, index: usize) -> &mut F {
		&mut self.v[index]
	}
}
