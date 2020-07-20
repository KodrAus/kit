//! A foundational layer for gamedev designed to be a starting point for jams.
//! Draws inspiration from ggez and Love2d, but can do 3D rendering.

#![allow(dead_code)]
#![allow(unused)]

pub mod geometry;
pub mod graphics;
mod math;

// re-exporting for convenient importing by consumers
pub use math::*;

// re-exporting for convenient obfuscation - I may replace sokol_app with winit
pub use sokol::app::SAppDesc as KAppDesc;
pub use sokol::app::SAppKeycode as Keycode;

use sokol::app::*;
use sokol::gfx::*;

// ----------------------------------------------------------------------------
// drawing structures and utils

// pub(crate) const BYTES_MODEL_BUFF_V (size_of::<MeshVert>() * MAX_MODEL_VERTS)
// pub(crate) const BYTES_MODEL_BUFF_I (size_of::<u32>() * MAX_MODEL_VERTS)
pub(crate) const MAX_QUADS: usize = 4000;
pub(crate) const MAX_POINTS: usize = 15000;
pub(crate) const MAX_LINES: usize = 1000;
pub(crate) const MAX_IMAGES: usize = 100;
pub(crate) const MAX_MESHES: usize = 200;

#[derive(Default, Copy, Clone)]
pub struct Texture {
  pub id: usize,
  pub w: u32,
  pub h: u32,
}

// TODO move to game layer?
#[derive(Default, Copy, Clone)]
pub struct TextureFrameDesc {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
  pub offset: V2,
}

/// Defines a subset of a texture that can be drawn. Can be used to define the
/// placement of a single sprite within a spritesheet.
///
/// Internally, stores uv coordinates related to the texture, so access to the
/// loaded texture must exist so that the full width and height of the texture
/// can be known. Note that uv coordinates are relative to the lower left corner
/// of the image.
///
/// TODO (wesh) consider whether it would be better to remove direct tie to the
/// texture_id in order to allow the same sprite dimensions to be reused with
/// different (equally sized) images.
#[derive(Default, Clone, Copy)]
pub struct Sprite {
  pub(crate) img_id: usize,
  pub(crate) corners: QuadCorners,
}

/// Primarily used for images, this expresses a point within the
/// image that will be aligned to the image's position coordinates
/// and which will be the center of any scaling or rotation applied
/// to the image.
///
/// Pivot point coordinates, like Sprites and Quad uvs, are relative
/// to the lower-left corner of the Sprite in question.
#[derive(Copy, Clone)]
pub enum Pivot {
  Center,
  Px(V2),
  Percent(V2),
}

#[derive(Default, Copy, Clone)]
pub(crate) struct DrawPoint {
  pub pos: V3,
  pub color: V4,
}

impl DrawPoint {
  pub fn new(x: f32, y: f32, z: f32, color: V4) -> DrawPoint {
    let pos = V3 { x, y, z };
    DrawPoint { pos, color }
  }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct DrawLine {
  pub point_a: V3,
  pub color_a: V4,
  pub point_b: V3,
  pub color_b: V4, // TODO do I *really* need gradient lines?
}

#[derive(Default, Copy, Clone)]
pub(crate) struct QuadVert {
  pub pos: V3,
  pub uv: V2,
}

impl QuadVert {
  pub fn new(x: f32, y: f32, z: f32, uvx: f32, uvy: f32) -> QuadVert {
    let pos = V3 { x, y, z };
    let uv = V2 { x: uvx, y: uvy };
    QuadVert { pos, uv }
  }
}

pub(crate) type QuadCorners = [QuadVert; 4];

#[derive(Default, Copy, Clone)]
pub(crate) struct DrawQuad {
  pub img_id: usize,
  pub corners: QuadCorners,
  pub transform: M4,
}

#[derive(Default, Clone, Copy)]
pub(crate) struct DrawMesh {
  pub mesh_i: usize,
  pub transform: M4,
}

#[derive(Default)]
pub(crate) struct GlShape {
  pub pipeline: SgPipeline,
  pub bindings: SgBindings,
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Image {
  pub(crate) e: SgImage,
  pub(crate) w: u32,
  pub(crate) h: u32,
}

pub(crate) struct ImagesCtx {
  e: [Image; MAX_IMAGES],
  count: usize,
}

impl Default for ImagesCtx {
  fn default() -> Self {
    Self {
      e: [Default::default(); MAX_IMAGES],
      count: 0,
    }
  }
}

pub(crate) struct QuadsCtx {
  pub(crate) shape: GlShape,
  pub(crate) e: [DrawQuad; MAX_QUADS],
  pub(crate) count: usize,
}

impl Default for QuadsCtx {
  fn default() -> Self {
    Self {
      shape: Default::default(),
      e: [Default::default(); MAX_QUADS],
      count: Default::default(),
    }
  }
}

pub(crate) struct PointsCtx {
  pub(crate) shape: GlShape,
  pub(crate) e: [DrawPoint; MAX_POINTS],
  pub(crate) count: usize,
}

impl Default for PointsCtx {
  fn default() -> Self {
    Self {
      shape: Default::default(),
      e: [Default::default(); MAX_POINTS],
      count: Default::default(),
    }
  }
}

pub(crate) struct LinesCtx {
  pub(crate) shape: GlShape,
  pub(crate) e: [DrawLine; MAX_LINES],
  pub(crate) count: usize,
}

impl Default for LinesCtx {
  fn default() -> Self {
    Self {
      shape: Default::default(),
      e: [Default::default(); MAX_LINES],
      count: Default::default(),
    }
  }
}

pub(crate) struct MeshCtx {
  pub(crate) shape: GlShape,
  pub(crate) e: [DrawMesh; MAX_MESHES],
  pub(crate) count: usize,
}

impl Default for MeshCtx {
  fn default() -> Self {
    Self {
      shape: Default::default(),
      e: [Default::default(); MAX_MESHES],
      count: Default::default(),
    }
  }
}

// TODO add api for setting bg, proj, and view and then hide the whole GraphicsCtx from the external api
#[derive(Default)]
pub struct GraphicsCtx {
  pub bg: V3,
  pub proj: M4,
  pub view: M4,
  pub(crate) view_proj: M4,
  //
  pub(crate) quads: QuadsCtx,
  pub(crate) points: PointsCtx,
  pub(crate) lines: LinesCtx,
  pub(crate) images: ImagesCtx,
  pub(crate) mesh: MeshCtx,
  //
  pub(crate) pass_action: SgPassAction,
}

/// Holds input state. Read from this during a game update to consume player inputs.
#[derive(Default)]
pub struct InputCtx {
  // TODO add multiple controllers
  pub l_stick: V2,
  pub r_stick: V2,

  // TODO should this be pub? Maybe hide it as an implementation detail
  pub quit: bool,
  // TODO replace these with actual keyboard state - what to do with the keys should be
  // a detail the game provides.
  pub dir_u: bool,
  pub dir_d: bool,
  pub dir_l: bool,
  pub dir_r: bool,
  pub action_pressed: bool,
  pub action_released: bool,
  //
  pub mouse_wheel_y: f32,
  // TODO mouse_wheel_x?

  // TODO should there be a way to get mouse position in world coordinates? ie. reverse view & projeection?
  pub mouse_pos: V2,
  pub mouse_prev_pos: V2,
}

// TODO should arrays in here be Vec<T> instead? Heap instead of stack?

#[derive(Default)]
pub struct Ctx {
  pub input: InputCtx,
  pub gl: GraphicsCtx,
}

// ----------------------------------------------------------------------------
// Lifecycle

struct App<K: KApp> {
  ctx: Ctx,
  app: K,
}

pub trait KApp: 'static + Sized {
  fn new() -> Self;
  fn init(&mut self, ctx: &mut Ctx);
  fn frame(&mut self, ctx: &mut Ctx);
}

impl<K: KApp> SApp for App<K> {
  fn sapp_init(&mut self) {
    let ctx = &mut self.ctx;
    graphics::init(ctx);
    self.app.init(ctx);
  }

  fn sapp_frame(&mut self) {
    let ctx = &mut self.ctx;
    ctx.gl.view_proj = ctx.gl.proj * ctx.gl.view;
    self.app.frame(ctx);
    ctx.input.mouse_wheel_y = 0.0;
    ctx.input.mouse_prev_pos = ctx.input.mouse_pos;
    graphics::present(ctx);
  }

  fn sapp_cleanup(&mut self) {
    std::process::exit(0);
  }

  fn sapp_event(&mut self, event: SAppEvent) {
    let ctx = &mut self.ctx;
    // check for system exit shortcut
    if event.event_type == SAppEventType::KeyDown
      && event.modifiers.contains(SAppModifier::SUPER)
      && (event.key_code == SAppKeycode::KeyW || event.key_code == SAppKeycode::KeyQ)
    {
      std::process::exit(0)
    }

    // TODO... sapp for events vs sdl? how do I handle gamepad input?
    match event.event_type {
      SAppEventType::MouseMove => {
        ctx.input.mouse_pos = v2(event.mouse_x, event.mouse_y);
      }
      SAppEventType::MouseScroll => {
        ctx.input.mouse_wheel_y += event.scroll_y;
      }
      SAppEventType::KeyDown => match event.key_code {
        SAppKeycode::KeyW => ctx.input.dir_u = true,
        SAppKeycode::KeyA => ctx.input.dir_l = true,
        SAppKeycode::KeyD => ctx.input.dir_r = true,
        SAppKeycode::KeyS => ctx.input.dir_d = true,
        _ => {}
      },
      SAppEventType::KeyUp => match event.key_code {
        SAppKeycode::KeyW => ctx.input.dir_u = false,
        SAppKeycode::KeyA => ctx.input.dir_l = false,
        SAppKeycode::KeyD => ctx.input.dir_r = false,
        SAppKeycode::KeyS => ctx.input.dir_d = false,
        _ => {}
      },

      _ => {}
    }
  }
}

pub fn run<K: KApp>(desc: KAppDesc) {
  let ctx: Ctx = Default::default();
  let app: K = K::new();
  sapp_run(App { ctx, app }, desc);
}

// ----------------------------------------------------------------------------
// Gamepad input

// TODO test these
// const AXIS_MIN: i16 = 4;
// const AXIS_MAX: i32 = 1000;

// fn get_normalized_gamepad_axis_value(controller: GameController, axis: Axis) -> f32 {
//   let value = controller.axis(axis);
//   let magnitude = value.abs();
//   if magnitude < AXIS_MIN {
//     return 0.0;
//   }
//   let normalized_magnitude = ((magnitude - AXIS_MIN) as f32) / (AXIS_MAX as f32);
//   if value > 0 {
//     normalized_magnitude
//   } else {
//     -normalized_magnitude
//   }
// }
