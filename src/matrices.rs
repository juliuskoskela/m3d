//! # Matrices
//!
//! This module contains the `Matrix` struct and its associated functions.
//!
//! ## Example
//!
//! ```
//! use math3d::matrices::Matrix;
//!
//! let matrix = Matrix::identity();
//!
//! println!("{}", matrix);
//! ```

use num::Float;

// //////////////////////////////////////////////////////////////////////////////////////
//
// Matrix3
//
// //////////////////////////////////////////////////////////////////////////////////////

use crate::vectors::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Matrix3<F: Float> {
    m: [Vector3<F>; 3],
}

impl<F: Float> Matrix3<F> {
    /// Create a new matrix from 9 values.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    ///
    /// assert_eq!(m[0][0], 1.0);
    /// assert_eq!(m[0][1], 2.0);
    /// ```

    pub fn new(
        m00: F,
        m01: F,
        m02: F,
        m10: F,
        m11: F,
        m12: F,
        m20: F,
        m21: F,
        m22: F,
    ) -> Matrix3<F> {
        Matrix3 {
            m: [
                Vector3::new(m00, m01, m02),
                Vector3::new(m10, m11, m12),
                Vector3::new(m20, m21, m22),
            ],
        }
    }

    /// Identity matrix.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::identity();
    ///
    /// assert_eq!(m[0][0], 1.0);
    /// assert_eq!(m[0][1], 0.0);
    /// ```

    pub fn identity() -> Matrix3<F> {
        let one = F::one();
        let zero = F::zero();
        Matrix3::new(one, zero, zero, zero, one, zero, zero, zero, one)
    }

    /// Create a matrix from 3 vectors.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_vectors(
    /// 	Vector3::new(1.0, 2.0, 3.0),
    /// 	Vector3::new(4.0, 5.0, 6.0),
    /// 	Vector3::new(7.0, 8.0, 9.0),
    /// );
    ///
    /// assert_eq!(m.m[0].x(), 1.0);
    /// assert_eq!(m.m[0].y(), 2.0);
    /// assert_eq!(m.m[0].z(), 3.0);
    /// assert_eq!(m.m[1].x(), 4.0);
    /// assert_eq!(m.m[1].y(), 5.0);
    /// assert_eq!(m.m[1].z(), 6.0);
    /// assert_eq!(m.m[2].x(), 7.0);
    /// assert_eq!(m.m[2].y(), 8.0);
    /// assert_eq!(m.m[2].z(), 9.0);
    /// ```

    pub fn from_vectors(v0: Vector3<F>, v1: Vector3<F>, v2: Vector3<F>) -> Matrix3<F> {
        Matrix3 { m: [v0, v1, v2] }
    }

    /// Create a matrix from [F; 9] array.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    ///
    /// assert_eq!(m.m[0].x(), 1.0);
    /// assert_eq!(m.m[0].y(), 2.0);
    /// assert_eq!(m.m[0].z(), 3.0);
    /// assert_eq!(m.m[1].x(), 4.0);
    /// assert_eq!(m.m[1].y(), 5.0);
    /// assert_eq!(m.m[1].z(), 6.0);
    /// assert_eq!(m.m[2].x(), 7.0);
    /// assert_eq!(m.m[2].y(), 8.0);
    /// assert_eq!(m.m[2].z(), 9.0);
    /// ```

    pub fn from_array(arr: [F; 9]) -> Matrix3<F> {
        Matrix3 {
            m: [
                Vector3::new(arr[0], arr[1], arr[2]),
                Vector3::new(arr[3], arr[4], arr[5]),
                Vector3::new(arr[6], arr[7], arr[8]),
            ],
        }
    }

    /// Convert a matrix to [F; 9] array.
    ///
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    /// let arr = m.to_array();
    ///
    /// assert_eq!(arr[0], 1.0);
    /// assert_eq!(arr[1], 2.0);
    /// ```

    pub fn to_array(&self) -> [F; 9] {
        [
            *self.m[0].x(),
            *self.m[0].y(),
            *self.m[0].z(),
            *self.m[1].x(),
            *self.m[1].y(),
            *self.m[1].z(),
            *self.m[2].x(),
            *self.m[2].y(),
            *self.m[2].z(),
        ]
    }

    /// Slice at index.
    ///
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    ///
    /// let m0 = m.index(0);
    ///
    /// assert_eq!(m0[0], 1.0);
    /// ```

    pub fn index(&self, i: usize) -> &F {
        let v = &self.m[i / 3];
        &v[i % 3]
    }

    /// Create a matrix from [[F; 3]; 3] array.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///

    pub fn from_array_2d(arr: [[F; 3]; 3]) -> Matrix3<F> {
        Matrix3 {
            m: [
                Vector3::new(arr[0][0], arr[0][1], arr[0][2]),
                Vector3::new(arr[1][0], arr[1][1], arr[1][2]),
                Vector3::new(arr[2][0], arr[2][1], arr[2][2]),
            ],
        }
    }

    /// Decompose tha matrix into a [[F; 3]; 3] array.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let arr = m.to_array_2d();
    ///
    /// assert_eq!(arr[0][0], 1.0);
    /// assert_eq!(arr[0][1], 2.0);
    /// assert_eq!(arr[0][2], 3.0);
    /// assert_eq!(arr[1][0], 4.0);
    /// assert_eq!(arr[1][1], 5.0);
    /// assert_eq!(arr[1][2], 6.0);
    /// assert_eq!(arr[2][0], 7.0);
    /// assert_eq!(arr[2][1], 8.0);
    /// assert_eq!(arr[2][2], 9.0);
    /// ```

    pub fn to_array_2d(&self) -> [[F; 3]; 3] {
        [
            [*self.m[0].x(), *self.m[0].y(), *self.m[0].z()],
            [*self.m[1].x(), *self.m[1].y(), *self.m[1].z()],
            [*self.m[2].x(), *self.m[2].y(), *self.m[2].z()],
        ]
    }

    /// To [f63; 9] array.
    ///
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    /// let arr = m.to_array_f64();
    ///
    /// assert_eq!(arr[0], 1.0);
    /// assert_eq!(arr[1], 2.0);
    /// ```

    pub fn to_array_f64(&self) -> [f64; 9] {
        [
            self.m[0][0].to_f64().unwrap(),
            self.m[0][1].to_f64().unwrap(),
            self.m[0][2].to_f64().unwrap(),
            self.m[1][0].to_f64().unwrap(),
            self.m[1][1].to_f64().unwrap(),
            self.m[1][2].to_f64().unwrap(),
            self.m[2][0].to_f64().unwrap(),
            self.m[2][1].to_f64().unwrap(),
            self.m[2][2].to_f64().unwrap(),
        ]
    }

    /// To [f32; 9] array.
    ///
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    /// let arr = m.to_array_f32();
    ///
    /// assert_eq!(arr[0], 1.0);
    /// assert_eq!(arr[1], 2.0);
    /// ```

    pub fn to_array_f32(&self) -> [f32; 9] {
        [
            self.m[0][0].to_f32().unwrap(),
            self.m[0][1].to_f32().unwrap(),
            self.m[0][2].to_f32().unwrap(),
            self.m[1][0].to_f32().unwrap(),
            self.m[1][1].to_f32().unwrap(),
            self.m[1][2].to_f32().unwrap(),
            self.m[2][0].to_f32().unwrap(),
            self.m[2][1].to_f32().unwrap(),
            self.m[2][2].to_f32().unwrap(),
        ]
    }

    /// Multiply two matrices.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m1 = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let m2 = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::from_array_2d([
    /// 	[30.0, 36.0, 42.0],
    /// 	[66.0, 81.0, 96.0],
    /// 	[102.0, 126.0, 150.0],]);
    ///
    /// assert!(m1 * m2 == expected);
    /// ```

    pub fn mul(self, other: Matrix3<F>) -> Matrix3<F> {
        let lhs = self.to_array_2d();
        let rhs = other.to_array_2d();

        let mut res = [[F::zero(); 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    res[i][j] = res[i][j] + lhs[i][k] * rhs[k][j];
                }
            }
        }
        Matrix3::from_array_2d(res)
    }

    /// Multiply matrix by scalar.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::from_array_2d([
    /// 	[2.0, 4.0, 6.0],
    /// 	[8.0, 10.0, 12.0],
    /// 	[14.0, 16.0, 18.0],]);
    ///
    /// assert!(m * 2.0 == expected);
    /// ```

    pub fn mul_scalar(self, scalar: F) -> Matrix3<F> {
        Matrix3 {
            m: [self.m[0] * scalar, self.m[1] * scalar, self.m[2] * scalar],
        }
    }

    /// Divide matrix by a matrix.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m1 = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let m2 = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::from_array_2d([
    /// 	[1.0, 1.0, 1.0],
    /// 	[1.0, 1.0, 1.0],
    /// 	[1.0, 1.0, 1.0],]);
    ///
    /// assert!(m1 / m2 == expected);
    /// ```

    pub fn div(self, other: Matrix3<F>) -> Matrix3<F> {
        let lhs = self.to_array_2d();
        let rhs = other.to_array_2d();

        let mut res = [[F::zero(); 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    res[i][j] = res[i][j] + lhs[i][k] * rhs[k][j];
                }
            }
        }
        Matrix3::from_array_2d(res)
    }

    /// Divide matrix by scalar.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::<f32>::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::<f32>::from_array_2d([
    /// 	[0.5, 1.0, 1.5],
    /// 	[2.0, 2.5, 3.0],
    /// 	[3.5, 4.0, 4.5],]);
    ///
    /// assert!(m / 2.0 == expected);
    /// ```

    pub fn div_scalar(self, scalar: F) -> Matrix3<F> {
        Matrix3 {
            m: [self.m[0] / scalar, self.m[1] / scalar, self.m[2] / scalar],
        }
    }

    /// Transpose matrix.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::from_array_2d([
    /// 	[1.0, 4.0, 7.0],
    /// 	[2.0, 5.0, 8.0],
    /// 	[3.0, 6.0, 9.0],]);
    ///
    /// assert!(m.transpose() == expected);
    /// ```

    pub fn transpose(self) -> Matrix3<F> {
        let mut m = self.to_array_2d();

        for i in 0..3 {
            for j in 0..3 {
                if i > j {
                    let tmp = m[i][j];
                    m[i][j] = m[j][i];
                    m[j][i] = tmp;
                }
            }
        }
        Matrix3::from_array_2d(m)
    }

    /// Get determinant of matrix.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// assert!(m.determinant() == 0.0);
    /// ```

    pub fn determinant(self) -> F {
        let m = self.to_array_2d();
        let mut res = F::zero();
        for i in 0..3 {
            res = res + m[0][i] * m[1][(i + 1) % 3] * m[2][(i + 2) % 3];
            res = res - m[0][i] * m[1][(i + 2) % 3] * m[2][(i + 1) % 3];
        }
        res
    }

    /// Get inverse of matrix.
    /// ```
    /// use math3d::matrices::Matrix3;
    ///
    /// let m = Matrix3::from_array_2d([
    /// 	[1.0, 2.0, 3.0],
    /// 	[4.0, 5.0, 6.0],
    /// 	[7.0, 8.0, 9.0],]);
    ///
    /// let expected = Matrix3::from_array_2d([
    /// 	[-2.0, 4.0, -2.0],
    /// 	[1.0, -5.0, 1.0],
    /// 	[2.0, -1.0, 2.0],]);
    ///
    /// assert!(m.inverse() == expected);
    /// ```

    pub fn inverse(self) -> Matrix3<F> {
        let mut m = self.to_array_2d();
        let mut res = Matrix3::<F>::identity().to_array_2d();
        let det = self.determinant();

        if det == F::zero() {
            return Matrix3::<F>::identity();
        }

        for i in 0..3 {
            for j in 0..3 {
                if i > j {
                    let tmp = m[i][j];
                    m[i][j] = m[j][i];
                    m[j][i] = tmp;
                    let tmp = res[i][j];
                    res[i][j] = res[j][i];
                    res[j][i] = tmp;
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                res[i][j] = res[i][j] / det;
            }
        }

        Matrix3::from_array_2d(res)
    }
}

impl<F: Float> core::fmt::Display for Matrix3<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let a = self.to_array_f64();
        write!(
            f,
            "[ {:.2} {:.2} {:.2} ]\n\n[ {:.2} {:.2} {:.2} ]\n\n[ {:.2} {:.2} {:.2} ]",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]
        )
    }
}
impl<F: Float> std::cmp::PartialEq for Matrix3<F> {
    fn eq(&self, other: &Matrix3<F>) -> bool {
        self.m[0] == other.m[0] && self.m[1] == other.m[1] && self.m[2] == other.m[2]
    }
}

impl<F: Float> std::ops::Mul for Matrix3<F> {
    type Output = Matrix3<F>;

    fn mul(self, rhs: Matrix3<F>) -> Matrix3<F> {
        self.mul(rhs)
    }
}

impl<F: Float> std::ops::Mul<F> for Matrix3<F> {
    type Output = Matrix3<F>;

    fn mul(self, rhs: F) -> Matrix3<F> {
        self.mul_scalar(rhs)
    }
}

impl<F: Float> std::ops::Div for Matrix3<F> {
    type Output = Matrix3<F>;

    fn div(self, rhs: Matrix3<F>) -> Matrix3<F> {
        self.div(rhs)
    }
}

impl<F: Float> std::ops::Div<F> for Matrix3<F> {
    type Output = Matrix3<F>;

    fn div(self, rhs: F) -> Matrix3<F> {
        self.div_scalar(rhs)
    }
}

impl<F: Float> std::ops::Index<usize> for Matrix3<F> {
    type Output = Vector3<F>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl<F: Float> std::ops::IndexMut<usize> for Matrix3<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

// //////////////////////////////////////////////////////////////////////////////////////
//
// Matrix4
//
// //////////////////////////////////////////////////////////////////////////////////////

use crate::vectors::Vector4;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4<F: Float> {
    m: [Vector4<F>; 4],
}

impl<F: Float> Matrix4<F> {
    /// New 4x4 matrix from 16 values.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::new(
    /// 	1.0, 2.0, 3.0, 4.0,
    /// 	5.0, 6.0, 7.0, 8.0,
    /// 	9.0, 10.0, 11.0, 12.0,
    /// 	13.0, 14.0, 15.0, 16.0);
    ///
    /// assert!(m.m[0][0] == 1.0);
    /// assert!(m.m[3][3] == 16.0);
    /// ```

    pub fn new(
        m00: F,
        m01: F,
        m02: F,
        m03: F,
        m10: F,
        m11: F,
        m12: F,
        m13: F,
        m20: F,
        m21: F,
        m22: F,
        m23: F,
        m30: F,
        m31: F,
        m32: F,
        m33: F,
    ) -> Matrix4<F> {
        Matrix4 {
            m: [
                Vector4::new(m00, m01, m02, m03),
                Vector4::new(m10, m11, m12, m13),
                Vector4::new(m20, m21, m22, m23),
                Vector4::new(m30, m31, m32, m33),
            ],
        }
    }

    /// New 4x4 identity matrix.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::identity();
    ///
    /// assert!(m.m[0][0] == 1.0);
    /// assert!(m.m[3][3] == 1.0);
    /// ```

    pub fn identity() -> Matrix4<F> {
        Matrix4 {
            m: [
                Vector4::new(F::one(), F::zero(), F::zero(), F::zero()),
                Vector4::new(F::zero(), F::one(), F::zero(), F::zero()),
                Vector4::new(F::zero(), F::zero(), F::one(), F::zero()),
                Vector4::new(F::zero(), F::zero(), F::zero(), F::one()),
            ],
        }
    }

	/// New 4x4 zero matrix.
	///
	/// ```
	/// use math3d::matrices::Matrix4;
	///
	/// let m = Matrix4::zero();
	///
	/// assert!(m.m[0][0] == 0.0);
	/// assert!(m.m[3][3] == 0.0);
	/// ```

	pub fn zero() -> Matrix4<F> {
		Matrix4 {
			m: [
				Vector4::new(F::zero(), F::zero(), F::zero(), F::zero()),
				Vector4::new(F::zero(), F::zero(), F::zero(), F::zero()),
				Vector4::new(F::zero(), F::zero(), F::zero(), F::zero()),
				Vector4::new(F::zero(), F::zero(), F::zero(), F::zero()),
			],
		}
	}

    /// New 4x4 matrix from array of 4 vectors.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::from_vectors(
    /// 	Vector4::new(1.0, 2.0, 3.0, 4.0),
    /// 	Vector4::new(5.0, 6.0, 7.0, 8.0),
    /// 	Vector4::new(9.0, 10.0, 11.0, 12.0),
    /// 	Vector4::new(13.0, 14.0, 15.0, 16.0));
    ///
    /// assert!(m.m[0][0] == 1.0);
    /// assert!(m.m[3][3] == 16.0);
    /// ```

    pub fn from_vectors(
        v0: Vector4<F>,
        v1: Vector4<F>,
        v2: Vector4<F>,
        v3: Vector4<F>,
    ) -> Matrix4<F> {
        Matrix4 {
            m: [v0, v1, v2, v3],
        }
    }

    /// New 4x4 matrix from array of 16 values.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::from_array([
    /// 	1.0, 2.0, 3.0, 4.0,
    /// 	5.0, 6.0, 7.0, 8.0,
    /// 	9.0, 10.0, 11.0, 12.0,
    /// 	13.0, 14.0, 15.0, 16.0]);
    ///
    /// assert!(m.m[0][0] == 1.0);
    /// assert!(m.m[3][3] == 16.0);
    /// ```

    pub fn from_array(m: [F; 16]) -> Matrix4<F> {
        Matrix4 {
            m: [
                Vector4::new(m[0], m[1], m[2], m[3]),
                Vector4::new(m[4], m[5], m[6], m[7]),
                Vector4::new(m[8], m[9], m[10], m[11]),
                Vector4::new(m[12], m[13], m[14], m[15]),
            ],
        }
    }

    /// Index into matrix.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::identity();
    ///
    /// assert!(m[0][0] == 1.0);
    /// assert!(m[3][3] == 1.0);
    /// ```

    pub fn index(&self, index: usize) -> &F {
        let v = &self.m[index / 4];
        &v[index % 4]
    }

    /// To array f32.
    ///
    /// ```
    /// use math3d::matrices::Matrix4;
    ///
    /// let m = Matrix4::identity();
    ///
    /// let af32 = m.to_array_f32();
    ///
    /// assert!(af32[0] == 1.0);
    /// assert!(af32[15] == 1.0);
    /// ```

    pub fn to_array_f32(&self) -> [f32; 16] {
        [
            self.m[0][0].to_f32().unwrap(),
            self.m[0][1].to_f32().unwrap(),
            self.m[0][2].to_f32().unwrap(),
            self.m[0][3].to_f32().unwrap(),
            self.m[1][0].to_f32().unwrap(),
            self.m[1][1].to_f32().unwrap(),
            self.m[1][2].to_f32().unwrap(),
            self.m[1][3].to_f32().unwrap(),
            self.m[2][0].to_f32().unwrap(),
            self.m[2][1].to_f32().unwrap(),
            self.m[2][2].to_f32().unwrap(),
            self.m[2][3].to_f32().unwrap(),
            self.m[3][0].to_f32().unwrap(),
            self.m[3][1].to_f32().unwrap(),
            self.m[3][2].to_f32().unwrap(),
            self.m[3][3].to_f32().unwrap(),
        ]
    }

	 /// To array f64
	 ///
	 /// ```
	 /// use math3d::matrices::Matrix4;
	 ///
	 /// let m = Matrix4::identity();
	 ///
	 /// let af64 = m.to_array_f64();
	 ///
	 /// assert!(af64[0] == 1.0);
	 /// assert!(af64[15] == 1.0);
	 /// ```

	 pub fn to_array_f64(&self) -> [f64; 16] {
		 [
			 self.m[0][0].to_f64().unwrap(),
			 self.m[0][1].to_f64().unwrap(),
			 self.m[0][2].to_f64().unwrap(),
			 self.m[0][3].to_f64().unwrap(),
			 self.m[1][0].to_f64().unwrap(),
			 self.m[1][1].to_f64().unwrap(),
			 self.m[1][2].to_f64().unwrap(),
			 self.m[1][3].to_f64().unwrap(),
			 self.m[2][0].to_f64().unwrap(),
			 self.m[2][1].to_f64().unwrap(),
			 self.m[2][2].to_f64().unwrap(),
			 self.m[2][3].to_f64().unwrap(),
			 self.m[3][0].to_f64().unwrap(),
			 self.m[3][1].to_f64().unwrap(),
			 self.m[3][2].to_f64().unwrap(),
			 self.m[3][3].to_f64().unwrap(),
		 ]
	 }

	/// To array of vectors.
	///
	/// ```
	/// use math3d::matrices::Matrix4;
	///
	/// let m = Matrix4::identity();
	///
	/// let av = m.to_array_vectors();
	///
	/// assert!(av[0][0] == 1.0);
	/// assert!(av[3][3] == 1.0);
	/// ```

	pub fn to_array_vectors(&self) -> [Vector4<F>; 4] {
		self.m
	}

	/// Multiply 4x4 matrix by 4x4 matrix.
	///
	/// ```
	/// use math3d::matrices::Matrix4;
	///
	/// let m1 = Matrix4::identity();
	/// let m2 = Matrix4::identity();
	///
	/// let m3 = m1 * m2;
	///
	/// assert!(m3 == m2);
	/// ```

	pub fn product(&self, other: Matrix4<F>) -> Matrix4<F> {
		let mut m = Matrix4::zero();

		for i in 0..4 {
			for j in 0..4 {
				for k in 0..4 {
					m[i][j] = m[i][j] + self[i][k] * other[k][j];
				}
			}
		}
		m
	}
}

impl<F: Float> core::fmt::Display for Matrix4<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let a = self.to_array_f64();
        write!(
            f,
            "[ {:.2} {:.2} {:.2} ]\n\n[ {:.2} {:.2} {:.2} ]\n\n[ {:.2} {:.2} {:.2} ]",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]
        )
    }
}
impl<F: Float> std::cmp::PartialEq for Matrix4<F> {
    fn eq(&self, other: &Matrix4<F>) -> bool {
        self.m[0] == other.m[0] && self.m[1] == other.m[1] && self.m[2] == other.m[2]
    }
}

impl<F: Float> std::ops::Mul for Matrix4<F> {
    type Output = Matrix4<F>;

    fn mul(self, rhs: Matrix4<F>) -> Matrix4<F> {
        self.product(rhs)
    }
}

impl<F: Float> std::ops::Mul<F> for Matrix4<F> {
    type Output = Matrix4<F>;

    fn mul(self, rhs: F) -> Matrix4<F> {
        todo!()
    }
}

impl<F: Float> std::ops::Div for Matrix4<F> {
    type Output = Matrix4<F>;

    fn div(self, rhs: Matrix4<F>) -> Matrix4<F> {
        todo!()
    }
}

impl<F: Float> std::ops::Div<F> for Matrix4<F> {
    type Output = Matrix4<F>;

    fn div(self, rhs: F) -> Matrix4<F> {
        todo!()
    }
}

impl<F: Float> std::ops::Index<usize> for Matrix4<F> {
    type Output = Vector4<F>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl<F: Float> std::ops::IndexMut<usize> for Matrix4<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}
