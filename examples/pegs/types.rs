use crate::board::Board;
use crate::constants::*;
use kit::*;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::time::Duration;

pub type EntityID = String;

#[derive(Copy, Clone)]
pub enum PegState {
  Idle,
  Excited,
  Buzz,
  Jump(Coords, Coords),
  Dying(Coords),
  Dead,
}

#[derive(Copy, Clone)]
pub enum PegType {
  Beige,
  Blue,
  Green,
  Pink,
  Yellow,
}

impl Distribution<PegType> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PegType {
    match rng.gen_range(0, 5) {
      0 => PegType::Beige,
      1 => PegType::Blue,
      2 => PegType::Green,
      3 => PegType::Pink,
      4 => PegType::Yellow,
      _ => panic!("not a valid random peg type!"),
    }
  }
}

#[derive(PartialEq)]
pub enum Phase {
  Picking,
  Excited(Coords),
  Jump,
  Reset,
}

impl Default for Phase {
  fn default() -> Self {
    // TODO should be ready, but starting in Picking for testing purposes
    Phase::Picking
  }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Coords {
  pub x: i32,
  pub y: i32,
}

pub fn coords(x: i32, y: i32) -> Coords {
  Coords { x, y }
}

impl Display for Coords {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "Coords({}, {})", self.x, self.y)
  }
}

impl Into<Vec2> for Coords {
  fn into(self) -> Vec2 {
    vec2(self.x as f32, self.y as f32)
  }
}

impl Add for Coords {
  type Output = Self;
  fn add(self, other: Coords) -> Self {
    coords(self.x + other.x, self.y + other.y)
  }
}

impl Sub for Coords {
  type Output = Self;
  fn sub(self, other: Coords) -> Self {
    coords(self.x - other.x, self.y - other.y)
  }
}

impl Mul<i32> for Coords {
  type Output = Self;
  fn mul(self, other: i32) -> Self {
    coords(self.x * other, self.y * other)
  }
}
#[derive(Default)]
pub struct Assets {
  pub reset: Option<Texture>,
  pub tile: Option<Texture>,
  pub shadow: Option<Texture>,
  pub peg_beige: Option<Texture>,
  pub peg_blue: Option<Texture>,
  pub peg_green: Option<Texture>,
  pub peg_pink: Option<Texture>,
  pub peg_yellow: Option<Texture>,
}

#[derive(Default)]
pub struct PegSheet {
  pub duck: Sprite,
  pub front: Sprite,
  pub hurt: Sprite,
  pub jump: Sprite,
  pub lean: Sprite,
}

#[derive(Default)]
pub struct Sprites {
  pub peg_beige: PegSheet,
  pub peg_blue: PegSheet,
  pub peg_green: PegSheet,
  pub peg_pink: PegSheet,
  pub peg_yellow: PegSheet,
}

pub struct Pegs {
  pub peg_type: [PegType; MAX_PEGS],
  pub state: [PegState; MAX_PEGS],
  pub animation: [u32; MAX_PEGS],
  pub z: [f32; MAX_PEGS],
  pub z_vel: [f32; MAX_PEGS],
  pub lean: [f32; MAX_PEGS],
}

impl Default for Pegs {
  fn default() -> Self {
    Self {
      peg_type: [PegType::Beige; MAX_PEGS],
      state: [PegState::Idle; MAX_PEGS],
      animation: [0; MAX_PEGS],
      z: [0.0; MAX_PEGS],
      z_vel: [0.0; MAX_PEGS],
      lean: [0.0; MAX_PEGS],
    }
  }
}

#[derive(Default)]
pub struct State {
  pub board: Board,             // mapping of board positions to peg_i
  pub assets: Assets,           // loaded assets for rendering
  pub sprites: Sprites,         // asset metadata for rendering
  pub phase: Phase,             // major game phase
  pub mouse_pos: Vec2,          // mouse position in world space
  pub over_peg: Option<Coords>, // peg coords if mouse is over one
  pub pegs: Pegs,               // peg properties types by peg_i
  pub reset_spin_animation: u32,
}
