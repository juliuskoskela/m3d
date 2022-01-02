use m3d::quaternion::Quaternion;
use m3d::vectors::Vector3;

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
fn test_quaternion_from_axis_angle() {
	let q1 = Quaternion::from_axis_angle(Vector3::from_array([1.0, 0.0, 0.0]), 90.0);
	let expected = Quaternion::new(0.7071067811865476, [0.7071067811865475, 0.0, 0.0]);
	assert!(q1 == expected);
}

#[test]
fn test_quaternion_from_euler() {
	let q1 = Quaternion::from_euler_angles(90.0, 0.0, 0.0);
	let expected = Quaternion::new(0.7071067811865476, [0.7071067811865475, 0.0, 0.0]);
	assert!(q1 == expected);
}

#[test]
fn test_quaternion_product() {
	let q1 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let q2 = Quaternion::new(1.0, [2.0, 3.0, 4.0]);
	let expected = Quaternion::new(-28.0, [4.0, 6.0, 8.0]);
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
	println!("{}", q3);
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
	todo!();
}
