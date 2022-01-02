use criterion::Throughput;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use m3d::points::Point3;
use m3d::vectors::Vector3;
use m3d::matrices::Matrix3;
use m3d::quaternion::Quaternion;
use rayon::prelude::*;
use rand::prelude::*;

pub struct Triangle {
	points: [Point3<f32>; 3],
}

fn random_triangles(count: usize) -> Vec<Triangle> {
	let mut rng = rand::thread_rng();
	let mut triangles = Vec::with_capacity(count);
	for _ in 0..count {
		let p1 = Point3::new(rng.gen(), rng.gen(), rng.gen());
		let p2 = Point3::new(rng.gen(), rng.gen(), rng.gen());
		let p3 = Point3::new(rng.gen(), rng.gen(), rng.gen());
		triangles.push(Triangle {
			points: [p1, p2, p3],
		});
	}
	triangles
}

/// Benchmark rotating a triangle by a quaternion 1000_000 times.

fn benchmark_triangle_rotation(
	c: &mut Criterion,
) {
	let mut group = c.benchmark_group("points");

	let rot = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), 90.0);

	let triangles = random_triangles(1000_000);

	group.throughput(Throughput::Elements(1000_000));

	group.bench_function("rotate_triangle", |b| {
		b.iter(|| {
			let rotated_triangles = triangles.iter().map(|tri| {
				Triangle {
					points: [
						Point3::from_vector(rot.rotate_vector(tri.points[0].to_vector())),
						Point3::from_vector(rot.rotate_vector(tri.points[1].to_vector())),
						Point3::from_vector(rot.rotate_vector(tri.points[2].to_vector())),
					],
				}
			});
			rotated_triangles.collect::<Vec<_>>()
		})
	});
	group.finish();
}

/// Benchmark rotating a triangle by a quaternion 1000_000 times.

fn benchmark_triangle_rotation_par(
	c: &mut Criterion,
) {
	let mut group = c.benchmark_group("points");

	let rot = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), 90.0);

	let triangles = random_triangles(1000_000);

	group.throughput(Throughput::Elements(1000_000));

	group.bench_function("rotate_triangle", |b| {
		b.iter(|| {
			let rotated_triangles = triangles.par_iter().map(|tri| {
				Triangle {
					points: [
						Point3::from_vector(rot.rotate_vector(tri.points[0].to_vector())),
						Point3::from_vector(rot.rotate_vector(tri.points[1].to_vector())),
						Point3::from_vector(rot.rotate_vector(tri.points[2].to_vector())),
					],
				}
			});
			rotated_triangles.collect::<Vec<_>>()
		})
	});
	group.finish();
}

fn benchmark_triangle_rotation_par_mat(
	c: &mut Criterion,
) {
	let mut group = c.benchmark_group("points");

	let rot = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), 90.0);

	let rot_matrix = rot.rotation_matrix();

	let triangles = random_triangles(1000_000);

	group.throughput(Throughput::Elements(1000_000));

	group.bench_function("rotate_triangle", |b| {
		b.iter( || {
			let rotated_triangles = triangles.par_iter().map(|tri| {
			});
			rotated_triangles.collect::<Vec<_>>()
		})
	});
	group.finish();
}

criterion_group!(
    benches,
	benchmark_triangle_rotation,
	benchmark_triangle_rotation_par,
	benchmark_triangle_rotation_par_mat
);

criterion_main!(benches);