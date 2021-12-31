use num::Float;
use crate::quaternion::Quaternion;
use crate::vectors::Vector3;

pub struct Point3<F: Float> {
	x: F,
	y: F,
	z: F,
}

impl<F :Float> Point3<F> {
	pub fn new(x: F, y: F, z: F) -> Point3<F> {
		Point3 { x, y, z }
	}

	pub fn x(&self) -> F {
		self.x
	}

	pub fn y(&self) -> F {
		self.y
	}

	pub fn z(&self) -> F {
		self.z
	}

	pub fn decompose(&self) -> [F; 3] {
		[self.x, self.y, self.z]
	}

	pub fn to_vector(&self) -> Vector3<F> {
		Vector3::new(self.x, self.y, self.z)
	}

	pub fn to_quaternion(&self, real: F) -> Quaternion<F> {
		Quaternion::new(real, [self.x, self.y, self.z])
	}

	pub fn distance(&self, other: &Point3<F>) -> F {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;
		(dx * dx + dy * dy + dz * dz).sqrt()
	}

	pub fn distance_squared(&self, other: &Point3<F>) -> F {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;
		dx * dx + dy * dy + dz * dz
	}

	pub fn rotate_axis_angle(&self, axis: [F; 3], angle: F) -> Point3<F> {
		let q = Quaternion::from_axis_angle(axis, angle);
		let v = self.to_vector();
		let v = q.rotate_vector(v);
		Point3::new(v.x(), v.y(), v.z())
	}
}