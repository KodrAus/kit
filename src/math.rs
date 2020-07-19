#![allow(dead_code)]

use core::ops::IndexMut;
pub use std::f32::consts::PI;
pub use std::f32::consts::SQRT_2;
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Sub;

// TODO TAU is in std in nightly - use std?
pub const TAU: f32 = 6.28318530717958647692528676655900577f32; // 6.2831853071795862f64

pub const INVERSE_SQ_ROOT_2: f32 = 1.0 / SQRT_2;

///////////////////////////////////////////////////////////////////////////////
// utils

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
  if a > b {
    a
  } else {
    b
  }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
  if a < b {
    a
  } else {
    b
  }
}

pub fn lerpf(a: f32, b: f32, f: f32) -> f32 {
  a + f * (b - a)
}

pub const NEAR_ZERO: f32 = 0.0000001; // TODO use f32::EPSILON?

pub fn near_zero(value: f32) -> bool {
  // TODO - use my own epsilon?
  value < NEAR_ZERO && value > -NEAR_ZERO
}

pub const fn v2(x: f32, y: f32) -> V2 {
  V2 { x, y }
}
pub const fn v3(x: f32, y: f32, z: f32) -> V3 {
  V3 { x, y, z }
}
pub const fn v4(x: f32, y: f32, z: f32, w: f32) -> V4 {
  V4 { x, y, z, w }
}

///////////////////////////////////////////////////////////////////////////////
// utils

pub fn to_radians(degrees: f32) -> f32 {
  // TODO is there a std lib function for this? If not, should I overload?
  degrees * (PI / 180.0) // TODO store PI / 180 as const?
}

///////////////////////////////////////////////////////////////////////////////
// vec 2

#[derive(Default, Copy, Clone)]
pub struct V2 {
  pub x: f32,
  pub y: f32,
}

impl Add for V2 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for V2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Mul<f32> for V2 {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

impl Div<f32> for V2 {
  type Output = Self;

  fn div(self, scalar: f32) -> Self {
    Self {
      x: self.x / scalar,
      y: self.y / scalar,
    }
  }
}

impl fmt::Display for V2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "V2({}, {})", self.x, self.y)
  }
}

impl V2 {
  pub const X: V2 = v2(1.0, 0.0);
  pub const Y: V2 = v2(0.0, 1.0);

  pub fn dot(self, b: V2) -> f32 {
    self.x * b.x + self.y * self.y
  }

  pub fn sq(self) -> f32 {
    self.x * self.x + self.y * self.y
  }

  pub fn mag(self: V2) -> f32 {
    f32::sqrt(self.sq())
  }

  pub fn abs(v: V2) -> V2 {
    v2(f32::abs(v.x), f32::abs(v.y))
  }

  pub fn aspect(v: V2) -> f32 {
    v.x / v.y
  }

  pub fn normalize(self) -> V2 {
    if self.x == 0.0 && self.y == 0.0 {
      self
    } else {
      self / self.mag()
    }
  }

  pub fn perp2(v: V2) -> V2 {
    v2(v.y, -v.x)
  }

  pub const LEFT: V2 = V2 { x: -1.0, y: 0.0 };
  pub const DOWN: V2 = V2 { x: 0.0, y: -1.0 };
  pub const RIGHT: V2 = V2 { x: 1.0, y: 0.0 };
  pub const UP: V2 = V2 { x: 0.0, y: 1.0 };
  pub const ZERO: V2 = V2 { x: 0.0, y: 0.0 };

  pub fn zeroish(self: V2) -> bool {
    near_zero(self.x) && near_zero(self.y)
  }
  pub fn nearly(a: V2, b: V2) -> bool {
    (a - b).zeroish()
  }
}

pub fn add2(a: V2, b: V2) -> V2 {
  V2 {
    x: a.x + b.x,
    y: a.y + b.y,
  }
}
pub fn sub2(a: V2, b: V2) -> V2 {
  V2 {
    x: a.x - b.x,
    y: a.y - b.y,
  }
}

pub fn mul2(v: V2, scalar: f32) -> V2 {
  V2 {
    x: v.x * scalar,
    y: v.y * scalar,
  }
}

///////////////////////////////////////////////////////////////////////////////
// vec 3

#[derive(Default, Copy, Clone)]
pub struct V3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl fmt::Display for V3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl V3 {
  pub const ZERO: V3 = v3(0.0, 0.0, 0.0);

  pub const X: V3 = v3(1.0, 0.0, 0.0);
  pub const Y: V3 = v3(0.0, 1.0, 0.0);
  pub const Z: V3 = v3(0.0, 0.0, 1.0);

  pub const fn new(x: f32, y: f32, z: f32) -> V3 {
    V3 { x, y, z }
  }

  pub fn to_arr4(&self) -> [f32; 4] {
    [self.x, self.y, self.z, 1.0]
  }
}

pub fn dot3(a: V3, b: V3) -> f32 {
  (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

pub fn cross3(a: V3, b: V3) -> V3 {
  V3 {
    x: (a.y * b.z) - (a.z * b.y),
    y: (a.z * b.x) - (a.x * b.z),
    z: (a.x * b.y) - (a.y * b.x),
  }
}

pub fn v3_mag_sq(v: V3) -> f32 {
  v.x * v.x + v.y * v.y + v.z * v.z
}

pub fn v3_mag(v: V3) -> f32 {
  f32::sqrt(v3_mag_sq(v))
}

pub fn v3_div(a: V3, scalar: f32) -> V3 {
  v3(a.x / scalar, a.y / scalar, a.z / scalar)
}

pub fn v3_norm(v: V3) -> V3 {
  v3_div(v, v3_mag(v))
}

impl Add for V3 {
  type Output = V3;
  fn add(self, b: V3) -> V3 {
    v3(self.x + b.x, self.y + b.y, self.z + b.z)
  }
}

impl Sub for V3 {
  type Output = V3;
  fn sub(self, b: V3) -> V3 {
    v3(self.x - b.x, self.y - b.y, self.z - b.z)
  }
}

impl Mul<f32> for V3 {
  type Output = V3;
  fn mul(self, scalar: f32) -> V3 {
    v3(self.x * scalar, self.y * scalar, self.z * scalar)
  }
}

pub fn v3_cross(a: V3, b: V3) -> V3 {
  V3 {
    x: (a.y * b.z) - (a.z * b.y),
    y: (a.z * b.x) - (a.x * b.z),
    z: (a.x * b.y) - (a.y * b.x),
  }
}

impl From<V2> for V3 {
  fn from(v: V2) -> Self {
    v3(v.x, v.y, 0.0)
  }
}

///////////////////////////////////////////////////////////////////////////////
// vec 4

#[derive(Default, Copy, Clone)]
pub struct V4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Index<usize> for V4 {
  type Output = f32;

  fn index(&self, i: usize) -> &Self::Output {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      3 => &self.w,
      _ => panic!("invalid index into V4"),
    }
  }
}

impl IndexMut<usize> for V4 {
  fn index_mut(&mut self, i: usize) -> &mut f32 {
    match i {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      3 => &mut self.w,
      _ => panic!("invalid index into V4"),
    }
  }
}

impl V4 {
  pub const ONE: V4 = V4::all(1.0);
  pub const ZERO: V4 = V4::all(0.0);

  pub const RED: V4 = v4(1.0, 0.0, 0.0, 1.0);
  pub const GEEN: V4 = v4(0.0, 1.0, 0.0, 1.0);
  pub const BLUE: V4 = v4(0.0, 0.0, 1.0, 1.0);

  pub const fn new(x: f32, y: f32, z: f32, w: f32) -> V4 {
    V4 { x, y, z, w }
  }

  pub const fn all(value: f32) -> V4 {
    V4::new(value, value, value, value)
  }

  pub fn mag_sq(v: V4) -> f32 {
    v.x * v.x + v.y * v.y + v.z * v.z + v.w * v.w
  }

  pub fn v4_mag(v: V4) -> f32 {
    f32::sqrt(V4::mag_sq(v))
  }
}

///////////////////////////////////////////////////////////////////////////////
// matrix 4x4

#[derive(Default, Clone, Copy)]
pub struct M4 {
  pub e: [[f32; 4]; 4],
}

impl M4 {
  pub const ZERO: M4 = M4::diag(0.0);
  pub const IDENTITY: M4 = M4::diag(1.0);

  pub fn mul(left: M4, right: M4) -> M4 {
    // TODO SSE
    let mut result: M4 = Default::default();
    for col in 0..4 {
      for row in 0..4 {
        let mut sum: f32 = 0.0;
        for current_matrice in 0..4 {
          sum += left.e[current_matrice][row] * right.e[col][current_matrice];
        }
        result.e[col][row] = sum;
      }
    }
    return result;
  }

  pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> M4 {
    let mut result: M4 = Default::default();
    let tan_theta_over2: f32 = f32::tan(fov * (PI / 360.0));
    result.e[0][0] = 1.0 / tan_theta_over2;
    result.e[1][1] = aspect / tan_theta_over2;
    result.e[2][3] = -1.0;
    result.e[2][2] = (near + far) / (near - far);
    result.e[3][2] = (2.0 * near * far) / (near - far);
    result.e[3][3] = 0.0;
    return result;
  }

  pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> M4 {
    let mut result: M4 = Default::default();

    result.e[0][0] = 2.0 / (right - left);
    result.e[1][1] = 2.0 / (top - bottom);
    result.e[2][2] = 2.0 / (near - far);
    result.e[3][3] = 1.0;

    result.e[3][0] = (left + right) / (left - right);
    result.e[3][1] = (bottom + top) / (bottom - top);
    result.e[3][2] = (far + near) / (near - far);

    return result;
  }

  pub const fn diag(diag: f32) -> M4 {
    let mut result: M4 = M4 { e: [[0.0; 4]; 4] };

    result.e[0][0] = diag;
    result.e[1][1] = diag;
    result.e[2][2] = diag;
    result.e[3][3] = diag;

    return result;
  }

  pub fn eq(a: M4, b: M4) -> bool {
    return a.e[0][0] == b.e[0][0]
      && a.e[0][1] == b.e[0][1]
      && a.e[0][2] == b.e[0][2]
      && a.e[0][3] == b.e[0][3]
      && a.e[1][0] == b.e[1][0]
      && a.e[1][1] == b.e[1][1]
      && a.e[1][2] == b.e[1][2]
      && a.e[1][3] == b.e[1][3]
      && a.e[2][0] == b.e[2][0]
      && a.e[2][1] == b.e[2][1]
      && a.e[2][2] == b.e[2][2]
      && a.e[2][3] == b.e[2][3]
      && a.e[3][0] == b.e[3][0]
      && a.e[3][1] == b.e[3][1]
      && a.e[3][2] == b.e[3][2]
      && a.e[3][3] == b.e[3][3];
  }

  pub fn rotate_rad(angle_radians: f32, axis: V3) -> M4 {
    let mut result: M4 = M4::IDENTITY;

    let axis = v3_norm(axis);

    let sin_theta: f32 = f32::sin(angle_radians);
    let cos_theta: f32 = f32::cos(angle_radians);
    let cos_value: f32 = 1.0 - cos_theta;

    result.e[0][0] = (axis.x * axis.x * cos_value) + cos_theta;
    result.e[0][1] = (axis.x * axis.y * cos_value) + (axis.z * sin_theta);
    result.e[0][2] = (axis.x * axis.z * cos_value) - (axis.y * sin_theta);

    result.e[1][0] = (axis.y * axis.x * cos_value) - (axis.z * sin_theta);
    result.e[1][1] = (axis.y * axis.y * cos_value) + cos_theta;
    result.e[1][2] = (axis.y * axis.z * cos_value) + (axis.x * sin_theta);

    result.e[2][0] = (axis.z * axis.x * cos_value) + (axis.y * sin_theta);
    result.e[2][1] = (axis.z * axis.y * cos_value) - (axis.x * sin_theta);
    result.e[2][2] = (axis.z * axis.z * cos_value) + cos_theta;

    return result;
  }

  pub fn rotate(angle: f32, axis: V3) -> M4 {
    let angle_radians: f32 = to_radians(angle);
    let result: M4 = M4::rotate_rad(angle_radians, axis);
    return result;
  }

  pub fn scale(scale: V3) -> M4 {
    let mut result: M4 = M4::diag(1.0);

    result.e[0][0] = scale.x;
    result.e[1][1] = scale.y;
    result.e[2][2] = scale.z;

    return result;
  }

  pub fn look_at(eye: V3, center: V3, up: V3) -> M4 {
    let mut result: M4 = Default::default();

    let f: V3 = v3_norm(center - eye);
    let s: V3 = v3_norm(v3_cross(f, up));
    let u: V3 = cross3(s, f);

    result.e[0][0] = s.x;
    result.e[0][1] = u.x;
    result.e[0][2] = -f.x;
    result.e[0][3] = 0.0;

    result.e[1][0] = s.y;
    result.e[1][1] = u.y;
    result.e[1][2] = -f.y;
    result.e[1][3] = 0.0;

    result.e[2][0] = s.z;
    result.e[2][1] = u.z;
    result.e[2][2] = -f.z;
    result.e[2][3] = 0.0;

    result.e[3][0] = -dot3(s, eye);
    result.e[3][1] = -dot3(u, eye);
    result.e[3][2] = dot3(f, eye);
    result.e[3][3] = 1.0;

    return result;
  }

  // pub fn M4::mul_v4(m: M4, v: V4) -> V4 {
  //   let result: V4;
  //   for rows in 0..4 {
  //     let mut sum: f32 = 0;
  //     for columns in 0..4 {
  //       sum += m.e[columns][rows] * v.e[columns];
  //     }
  //     result.e[rows] = sum;
  //   }
  //   return result;
  // }

  pub fn pos(m: M4) -> V3 {
    let x = m.e[3][0];
    let y = m.e[3][1];
    let z = m.e[3][2];
    V3 { x, y, z }
  }

  pub fn translate(v: V3) -> M4 {
    let mut result: M4 = M4::diag(1.0);
    result.e[3][0] = v.x;
    result.e[3][1] = v.y;
    result.e[3][2] = v.z;
    return result;
  }

  pub fn mul_by_v4(m: M4, v: V4) -> V4 {
    let mut result: V4 = V4::ZERO;
    for row in 0..4 {
      let mut sum = 0.0;
      for col in 0..4 {
        sum += m.e[col][row] * v[col];
      }
      result[row] = sum;
    }
    result
  }
}

impl Mul for M4 {
  type Output = Self;
  fn mul(self, rhs: M4) -> M4 {
    M4::mul(self, rhs)
  }
}
