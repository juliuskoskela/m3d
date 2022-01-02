//!
pub mod vectors;
pub mod quaternion;
pub mod matrices;
pub mod points;

#[cfg(test)]
mod tests {
	use crate::quaternion::Quaternion;

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