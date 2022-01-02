use crate::points::Point3;
use crate::vectors::Vector4;
use crate::matrices::Matrix4;
use crate::quaternion::Quaternion;
use num::Float;

pub fn translation<F: Float>(x: F, y: F, z: F) -> Matrix4<F> {
	let zero = F::zero();
	let one = F::one();

	Matrix4::from_vectors(
		Vector4::new(one, zero, zero, x),
		Vector4::new(zero, one, zero, y),
		Vector4::new(zero, zero, one, z),
		Vector4::new(zero, zero, zero, one),
	).transpose()
}

pub fn scale<F: Float>(x: F, y: F, z: F) -> Matrix4<F> {
	let zero = F::zero();
	let one = F::one();

	Matrix4::from_vectors(
		Vector4::new(x, zero, zero, zero),
		Vector4::new(zero, y, zero, zero),
		Vector4::new(zero, zero, z, zero),
		Vector4::new(zero, zero, zero, one),
	)
}

fn projection_<F: Float>(fov: F, aspect: F, near: F, far: F) -> Matrix4<F> {
	let zero = F::zero();
	let one = F::one();
	let two = F::one() + F::one();
	let f = one / (fov / two).tan();

	Matrix4::new(
		f / aspect, zero, zero, zero,
		zero, f, zero, zero,
		zero, zero, (far + near) / (near - far), (two * far * near) / (near - far),
		zero, zero, -one, zero,
	).transpose()
}

pub struct Camera<F: Float> {
	position: Point3<F>,
	rotation: Quaternion<F>,
	fov: F,
	aspect: F,
	near: F,
	far: F,
}

impl<F: Float> Camera<F> {

	// Creates a new camera.
	pub fn new(position: Point3<F>, rotation: Quaternion<F>, fov: F, aspect: F, near: F, far: F) -> Camera<F> {
		Camera {
			position,
			rotation,
			fov,
			aspect,
			near,
			far,
		}
	}

	// Returns the camera's position.
	pub fn position(&self) -> &Point3<F> {
		&self.position
	}

	// Updates the camera's position.
	pub fn update_position(&mut self, position: Point3<F>) {
		self.position = position;
	}

	// Returns the camera's rotation.
	pub fn rotation(&self) -> &Quaternion<F> {
		&self.rotation
	}

	// Updates the camera's rotation.
	pub fn update_rotation(&mut self, rotation: Quaternion<F>) {
		self.rotation = rotation;
	}

	// Returns the camera's field of view.
	pub fn fov(&self) -> &F {
		&self.fov
	}

	// Updates the camera's field of view.
	pub fn update_fov(&mut self, fov: F) {
		self.fov = fov;
	}

	// Returns the camera's aspect ratio.
	pub fn aspect(&self) -> &F {
		&self.aspect
	}

	// Updates the camera's aspect ratio.
	pub fn update_aspect(&mut self, aspect: F) {
		self.aspect = aspect;
	}

	// Returns the camera's near plane.
	pub fn near(&self) -> &F {
		&self.near
	}

	// Updates the camera's near plane.
	pub fn update_near(&mut self, near: F) {
		self.near = near;
	}

	// Returns the camera's far plane.
	pub fn far(&self) -> &F {
		&self.far
	}

	// Updates the camera's far plane.
	pub fn update_far(&mut self, far: F) {
		self.far = far;
	}

	// Returns the camera's view matrix.
	pub fn view(&self) -> Matrix4<F> {
		let zero = F::zero();
		let one = F::one();

		let translation = translation(self.position[0], self.position[1], self.position[2]);
		let rotation = self.rotation.rotation_matrix();
		let rot4x4 = Matrix4::from_vectors(
			Vector4::new(rotation[0][0], rotation[0][1], rotation[0][2], zero),
			Vector4::new(rotation[1][0], rotation[1][1], rotation[1][2], zero),
			Vector4::new(rotation[2][0], rotation[2][1], rotation[2][2], zero),
			Vector4::new(zero, zero, zero, one),
		);
		translation * rot4x4
	}

	// Returns the camera's projection matrix.
	pub fn projection(&self) -> Matrix4<F> {
		projection_(self.fov, self.aspect, self.near, self.far)
	}
}