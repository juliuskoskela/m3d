use crate::vectors::Vec3;
use crate::matrices::Mat3;

pub struct Camera {
	pub origin: Vec3,
	pub lower_left_corner: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub u: Vec3,
	pub v: Vec3,
	pub w: Vec3,
	pub lens_radius: f32,
}