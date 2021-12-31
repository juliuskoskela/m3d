pub mod vectors;
pub mod matrices;
pub mod points;
pub mod quaternion;

#[cfg(test)]
mod tests {
	use crate::matrices::Mat3;
	use crate::vectors::Vec3;
	use crate::quaternion::Quaternion;

	#[test]
	fn test_mat3() {
		let m1 = Mat3::new_from([
			1.0, 2.0, 3.0,
			4.0, 5.0, 6.0,
			7.0, 8.0, 9.0
		]);

		let m2 = Mat3::new_from([
			1.0, 4.0, 7.0,
			2.0, 5.0, 8.0,
			3.0, 6.0, 9.0
		]);

		let m3 = m1 * m2;
		m3.print();

		let v1 = Vec3::new(1.0, 2.0, 3.0);

		print!("{}", v1);

		let v2 = m1 * v1;

		print!("{}", v2);
	}

	#[test]
	// Unit test for Mat3::identity()
	fn test_identity() {
		let m1 = Mat3::identity();
		let m2 = Mat3::identity();
		assert!(m1 == m2);
	}

	#[test]
	// Unit test for Mat3::mul()
	fn test_mul() {
		let m1 = Mat3::new_from([
			1.0, 2.0, 3.0,
			4.0, 5.0, 6.0,
			7.0, 8.0, 9.0
		]);

		let m2 = Mat3::new_from([
			1.0, 4.0, 7.0,
			2.0, 5.0, 8.0,
			3.0, 6.0, 9.0
		]);

		let m1xm2 = Mat3::new_from([
			14.0, 32.0, 50.0,
			32.0, 77.0, 122.0,
			50.0, 122.0, 194.0,
		]);

		let m3 = m1 * m2;

		m2.print();
		assert!(m3 == m1xm2);

		let m4 = m2.transpose();

		m4.print();

		let m5 = m4 * 0.5;

		m5.print();
	}

	#[test]
	fn test_quaternion_identity() {
		let q1 = Quaternion::<f32>::identity();
		let q2 = Quaternion::<f32>::identity();
		assert!(q1 == q2);
	}

	#[test]
	fn test_quaternion_sum() {
		let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
		let q2 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
		let expected = Quaternion::new(2.0, [4.0, 6.0, 8.0]);
		let q3 = q1 + q2;
		assert!(q3 == expected);
	}

	#[test]
	fn test_quaternion_sub() {
		let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
		let q2 = Quaternion::new(5.0, [6.0, 7.0, 8.0]);
		let expected = Quaternion::new(-4.0, [-4.0, -4.0, -4.0]);
		let q3 = q1 - q2;
		assert!(q3 == expected);
	}

	#[test]
	fn test_quarternion_conjugate() {
		let q1 = Quaternion::new(7.0, [3.0, 23.0, 11.0]);
		let expected = Quaternion::new(7.0, [-3.0, -23.0, -11.0]);
		let q2 = q1.conjugate();
		assert!(q2 == expected);
	}

	#[test]
	fn test_quarternion_product() {
		let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
		let q2 = Quaternion::new(5.0, [6.0, 7.0, 8.0]);
		let expected = Quaternion::new(-60.0, [12.0, 30.0, 24.0]);
		let q3 = q1 * q2;
		assert!(q3 == expected);
	}

	#[test]
	fn test_quarternion_div() {
		let q1 = Quaternion::new(1.0, [0.0, 1.0, 0.0]);
		let q2 = Quaternion::new(1.0, [0.5, 0.5, 0.75]);
		let q3 = q1 / q2;
		println!("{}", q3);
	}
}