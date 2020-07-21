#![allow(dead_code)]

pub use glam::*;
pub use std::f32::consts::PI;
pub use std::f32::consts::SQRT_2;

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

pub fn to_radians(degrees: f32) -> f32 {
  // TODO is there a std lib function for this? If not, should I overload?
  degrees * (PI / 180.0) // TODO store PI / 180 as const?
}

///////////////////////////////////////////////////////////////////////////////
// vec 2

pub fn vec2_zeroish(v: Vec2) -> bool {
  near_zero(v.x()) && near_zero(v.y())
}

pub fn vec2_nearly(a: Vec2, b: Vec2) -> bool {
  vec2_zeroish(a - b)
}
