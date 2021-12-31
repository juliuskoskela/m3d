use crate::vectors::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
	pub m: [f32; 9],
}

impl Mat3 {
	pub fn new() -> Mat3 {
		Mat3 { m: [0.0; 9] }
	}

	pub fn new_from(m: [f32; 9]) -> Mat3 {
		Mat3 { m }
	}

	pub fn identity() -> Mat3 {
		Mat3 { m: [
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0
		] }
	}

	// Transpose the matrix  from row major to column major
	pub fn transpose(&self) -> Mat3 {
		let mut m = Mat3::new();
		// Get column major from row major
		for i in 0..3 {
			for j in 0..3 {
				m.m[i * 3 + j] = self.m[j * 3 + i];
			}
		}
		m
	}

	pub fn determinant(&self) -> f32 {
		self.m[0] * (self.m[4] * self.m[8] - self.m[5] * self.m[7])
			- self.m[1] * (self.m[3] * self.m[8] - self.m[5] * self.m[6])
			+ self.m[2] * (self.m[3] * self.m[7] - self.m[4] * self.m[6])
	}

	pub fn inverse(&self) -> Mat3 {
		let mut m = Mat3::new();
		let mut det = self.determinant();
		if det == 0.0 {
			return m;
		}
		det = 1.0 / det;
		m.m[0] = self.m[4] * self.m[8] - self.m[5] * self.m[7];
		m.m[1] = self.m[2] * self.m[7] - self.m[1] * self.m[8];
		m.m[2] = self.m[1] * self.m[5] - self.m[2] * self.m[4];
		m.m[3] = self.m[5] * self.m[6] - self.m[3] * self.m[8];
		m.m[4] = self.m[0] * self.m[8] - self.m[2] * self.m[6];
		m.m[5] = self.m[2] * self.m[3] - self.m[0] * self.m[5];
		m.m[6] = self.m[3] * self.m[7] - self.m[4] * self.m[6];
		m.m[7] = self.m[1] * self.m[6] - self.m[0] * self.m[7];
		m.m[8] = self.m[0] * self.m[4] - self.m[1] * self.m[3];
		m = m * det;
		m
	}

	// Trace is the sum of the diagonal elements
	pub fn trace(&self) -> f32 {
		self.m[0] + self.m[4] + self.m[8]
	}

	pub fn print(&self) {
		println!("");
		println!("[{:4}] [{:4}] [{:4}]", self.m[0], self.m[1], self.m[2]);
		println!("");
		println!("[{:4}] [{:4}] [{:4}]", self.m[3], self.m[4], self.m[5]);
		println!("");
		println!("[{:4}] [{:4}] [{:4}]", self.m[6], self.m[7], self.m[8]);
		println!("");
	}
}

impl std::ops::Mul for Mat3 {
	type Output = Mat3;

	fn mul(self, other: Mat3) -> Self::Output {
		let mut result = Mat3::new();
		for i in 0..3 {
			for j in 0..3 {
				result.m[i * 3 + j] = self.m[i * 3 + 0] * other.m[0 * 3 + j]
					+ self.m[i * 3 + 1] * other.m[1 * 3 + j]
					+ self.m[i * 3 + 2] * other.m[2 * 3 + j];
			}
		}
		result
	}
}

impl std::ops::Mul<Vector3> for Mat3 {
	type Output = Vector3;

	fn mul(self, other: Vector3) -> Self::Output {
		Vector3 { v: [
			self.m[0] * other.x() + self.m[1] * other.y() + self.m[2] * other.z(),
			self.m[3] * other.x() + self.m[4] * other.y() + self.m[5] * other.z(),
			self.m[6] * other.x() + self.m[7] * other.y() + self.m[8] * other.z()
		] }
	}
}

impl std::ops::Mul<f32> for Mat3 {
	type Output = Mat3;

	fn mul(self, other: f32) -> Self::Output {
		let mut result = Mat3::new();
		for i in 0..9 {
			result.m[i] = self.m[i] * other;
		}
		result
	}
}

impl std::ops::Div for Mat3 {
	type Output = Mat3;

	fn div(self, other: Mat3) -> Self::Output {
		let mut result = Mat3::new();
		for i in 0..3 {
			for j in 0..3 {
				result.m[i * 3 + j] = self.m[i * 3 + 0] / other.m[0 * 3 + j]
					+ self.m[i * 3 + 1] / other.m[1 * 3 + j]
					+ self.m[i * 3 + 2] / other.m[2 * 3 + j];
			}
		}
		result
	}
}

impl std::ops::Add for Mat3 {
	type Output = Mat3;

	fn add(self, other: Mat3) -> Self::Output {
		let mut result = Mat3::new();
		for i in 0..9 {
			result.m[i] = self.m[i] + other.m[i];
		}
		result
	}
}

impl std::ops::Sub for Mat3 {
	type Output = Mat3;

	fn sub(self, other: Mat3) -> Self::Output {
		let mut result = Mat3::new();
		for i in 0..9 {
			result.m[i] = self.m[i] - other.m[i];
		}
		result
	}
}

impl std::cmp::PartialEq for Mat3 {
	fn eq(&self, other: &Mat3) -> bool {
		for i in 0..9 {
			if self.m[i] != other.m[i] {
				return false;
			}
		}
		true
	}
}
