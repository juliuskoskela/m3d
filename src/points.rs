use num::Float;
use crate::quaternion::Quaternion;
use crate::vectors::Vector3;

pub struct Point3<F: Float> {
	xyz: Vector3<F>,
}

impl<F :Float> Point3<F> {

	/// Creates a new point.
	///
	/// # Arguments
	///
	/// * `x` - The x component of the point.
	/// * `y` - The y component of the point.
	/// * `z` - The z component of the point.
	///
	/// # Example
	///
	/// ```
	/// use math3d::points::Point3;
	///
	/// let point = Point3::new(1.0, 2.0, 3.0);
	/// ```

	pub fn new(x: F, y: F, z: F) -> Point3<F> {
		Point3 {
			xyz: Vector3::new(x, y, z),
		}
	}

	/// Creates a new point from a vector.
	///
	/// # Arguments
	///
	/// * `vector` - The vector to create the point from.
	///
	/// # Example
	///
	/// ```
	/// use math3d::points::Point3;
	///
	/// let vector = Vector3::new(1.0, 2.0, 3.0);
	/// let point = Point3::from_vector(vector);
	/// ```

	pub fn from_vector(vector: Vector3<F>) -> Point3<F> {
		Point3 {
			xyz: vector,
		}
	}

	/// To vector.
	///
	/// # Example
	///
	/// ```
	/// use math3d::points::Point3;
	///
	/// let point = Point3::new(1.0, 2.0, 3.0);
	/// let vector = point.to_vector();
	/// ```

	pub fn to_vector(&self) -> Vector3<F> {
		self.xyz
	}

	/// Distance to another point.
	///
	/// # Arguments
	///
	/// * `other` - The other point to calculate the distance to.
	///
	/// # Example
	///
	/// ```
	/// use math3d::points::Point3;
	///
	/// let point = Point3::new(1.0, 2.0, 3.0);
	/// let other = Point3::new(4.0, 5.0, 6.0);
	///
	/// assert_eq!(point.distance_to(other), 5.196152422706632);
	/// ```

	pub fn distance_to(&self, other: Point3<F>) -> F {
		(self.xyz - other.xyz).magnitude()
	}

	/// Rotates the point around the euler angles.
	///
	/// # Arguments
	///
	/// * `x` - The x angle to rotate the point by.
	/// * `y` - The y angle to rotate the point by.
	/// * `z` - The z angle to rotate the point by.
	///
	/// # Example
	///
	/// ```
	/// use math3d::points::Point3;
	///
	/// let point = Point3::new(1.0, 2.0, 3.0);
	/// let rotated_point = point.rotate_euler(90.0, 90.0, 90.0);
	///
	/// assert_eq!(rotated_point.x(), 2.0);
	/// assert_eq!(rotated_point.y(), 3.0);
	/// assert_eq!(rotated_point.z(), 1.0);
	/// ```

	pub fn rotate_euler(&self, x: F, y: F, z: F) -> Point3<F> {
		let quaternion = Quaternion::from_euler_angles(x, y, z);
		let rotated_point = quaternion.rotate_vector(self.xyz);
		Point3::from_vector(rotated_point)
	}

	pub fn rotate(&self, quaternion: Quaternion<F>) -> Point3<F> {
		let rotated_point = quaternion.rotate_vector(self.xyz);
		Point3::from_vector(rotated_point)
	}

	pub fn normalize(&self) -> Point3<F> {
		Point3::from_vector(self.xyz.normalized())
	}
}

impl<F: Float> core::fmt::Display for Point3<F> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "p: [{:.4}, {:.4}, {:.4}]", self[0].to_f64().unwrap(), self[1].to_f64().unwrap(), self[2].to_f64().unwrap())
	}
}

impl<F: Float> core::cmp::PartialEq for Point3<F> {
	fn eq(&self, other: &Point3<F>) -> bool {
		self.xyz == other.xyz
	}
}

impl<F: Float> std::ops::Index<usize> for Point3<F> {
	type Output = F;

	fn index(&self, index: usize) -> &F {
		&self.xyz[index]
	}
}