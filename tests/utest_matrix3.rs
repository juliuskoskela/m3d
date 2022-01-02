use math3d::matrices::Matrix3;

#[cfg(test)]

#[test]
fn test_matrix3_mul() {
	let m1 = Matrix3::from_array_2d([
		[1.0, 2.0, 3.0],
		[4.0, 5.0, 6.0],
		[7.0, 8.0, 9.0],
	]);
	let m2 = Matrix3::from_array_2d([
		[1.0, 2.0, 3.0],
		[4.0, 5.0, 6.0],
		[7.0, 8.0, 9.0],
	]);
	let e = Matrix3::from_array_2d([
		[30.0, 36.0, 42.0],
		[66.0, 81.0, 96.0],
		[102.0, 126.0, 150.0],
	]);

	let m3 = m1 * m2;

	assert!(m3 == e);
}