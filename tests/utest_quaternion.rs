use crate::quaternion::Quaternion;
use crate::vectors::Vector3;

// Create unit tests for Quarternion
#[cfg(test)]

#[test]
fn test_quaternion_new() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let q2 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	assert!(q1 == q2);
}

#[test]
fn test_quaternion_identity() {
	let q1 = Quaternion::<f32>::identity();
	let q2 = Quaternion::identity();
	assert!(q1 == q2);
}

#[test]
fn test_quaternion_product() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let q2 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let expected = Quaternion::new(1.0, [4.0, 6.0, 8.0]);
	let q3 = q1 * q2;
	assert!(q3 == expected);
}

#[test]
fn test_quaternion_conjugate() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let expected = Quaternion::new(1.0, [-2.0, -3.0, -4.0]);
	let q2 = q1.conjugate();
	assert!(q2 == expected);
}

#[test]
fn test_quaternion_div() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let q2 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let expected = Quaternion::new(1.0, [0.5, 0.5, 0.5]);
	let q3 = q1 / q2;
	assert!(q3 == expected);
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
fn test_quaternion_exp() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let expected = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let q2 = q1.exp();
	assert!(q2 == expected);
}

#[test]
fn test_quaternion_rotate_vector() {
	let q1 = Quaternion::new(0.0, [0.0, 1.0, 0.0]);
	let v1 = Vector3::new(0.0, 1.0, 0.0);
	let v2 = q1.rotate_vector(v1);
	println!("{}", v2);
}

