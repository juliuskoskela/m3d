use m3d::vectors::Vector3;

#[cfg(test)]

#[test]
fn test_vector3_new() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(v[0], 1.0);
	assert_eq!(v[1], 2.0);
}

#[test]
fn test_vector3_from_array() {
	let v = Vector3::from_array([1.0, 2.0, 3.0]);
	assert_eq!(v[0], 1.0);
	assert_eq!(v[1], 2.0);
}

#[test]
fn test_vector3_decompose() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	let (x, y, z) = v.decompose();
	assert_eq!(x, 1.0);
	assert_eq!(y, 2.0);
	assert_eq!(z, 3.0);
}

#[test]
fn test_vector3_x() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(v[0], 1.0);
}

#[test]
fn test_vector3_y() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(v[1], 2.0);
}

#[test]
fn test_vector3_z() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(v[2], 3.0);
}

#[test]
fn test_vector3_add() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 3.0, 4.0);
	let v3 = v1 + v2;
	assert_eq!(v3[0], 3.0);
	assert_eq!(v3[1], 5.0);
	assert_eq!(v3[2], 7.0);
}

#[test]
fn test_vector3_sub() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 3.0, 4.0);
	let v3 = v1 - v2;
	assert_eq!(v3[0], -1.0);
	assert_eq!(v3[1], -1.0);
	assert_eq!(v3[2], -1.0);
}

#[test]
fn test_vector3_mul() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 3.0, 4.0);
	let v3 = v1 * v2;
	assert_eq!(v3[0], 2.0);
	assert_eq!(v3[1], 6.0);
	assert_eq!(v3[2], 12.0);
}

#[test]
fn test_vector3_div() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 4.0, 6.0);
	let v3 = v1 / v2;
	assert_eq!(v3[0], 0.5);
	assert_eq!(v3[1], 0.5);
	assert_eq!(v3[2], 0.5);
}

#[test]
fn test_vector3_dot() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 3.0, 4.0);
	let v3 = v1.dot(v2);
	assert_eq!(v3, 20.0);
}

#[test]
fn test_vector3_cross() {
	let v1 = Vector3::new(1.0, 2.0, 3.0);
	let v2 = Vector3::new(2.0, 3.0, 4.0);
	let v3 = v1.cross(v2);
	assert_eq!(v3[0], -1.0);
	assert_eq!(v3[1], 2.0);
	assert_eq!(v3[2], -1.0);
}

#[test]
fn test_vector3_magnitude() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(v.magnitude(), 3.7416573867739413);
}

#[test]
fn test_vector3_normalize() {
	let v = Vector3::new(1.0, 2.0, 3.0);
	let v2 = v.normalized();
	assert_eq!(v2[0], 0.2672612419124244);
	assert_eq!(v2[1], 0.5345224838248488);
	assert_eq!(v2[2], 0.8017837257372732);
}
