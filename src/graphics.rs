//! Graphics commands for drawing primitives.
mod circle;
mod line;
mod mesh;
mod point;
mod quad;
mod rect;
mod sprite;

use crate::geometry::*;
use crate::math::*;
use crate::*;
use core::mem::size_of;
use image;
use image::*;
use std::f32;
use std::path::Path;

const BYTES_POINTS: usize = size_of::<DrawPoint>() * MAX_POINTS;
const BYTES_LINES: usize = size_of::<DrawLine>() * MAX_LINES;

// struct MeshVsParams {
//     model: Mat4,
//     view_proj: Mat4,
// }

// ----------------------------------------------------------------------------
// draw calls
//
// these build on the drawing primitives to draw special game structures like
// sprite sheets and circles

pub use circle::draw_circ;
pub use line::draw_line;
pub use mesh::draw_mesh;
pub use point::draw_point;
pub use rect::draw_rect;
pub use sprite::*;

// TODO reimplement
// pub fn draw_mesh(&mut self, mesh_i: u8, transform: Mat4) {
//   let i = self.num_meshes;
//   self.num_meshes += 1;
//   self.meshes[i] = DrawMesh { mesh_i, transform };
// }

/// general draw call for drawing a shape primitive

pub fn draw_shape(ctx: &mut Ctx, shape: Shape, color: Vec4) {
  match shape {
    Shape::Point(p) => draw_point(ctx, p.extend(0.0), color),
    Shape::Rect(r) => draw_rect(ctx, r, color),
    Shape::Circle(c) => draw_circ(ctx, c, color),
  }
}

// ----------------------------------------------------------------------------
// GETTERS

/// the current window width

pub fn window_width(_: &Ctx) -> f32 {
  (sapp_width() as f32)
}

/// the current window height

pub fn window_height(_: &Ctx) -> f32 {
  (sapp_height() as f32)
}

/// the current aspect ratio of the application window

pub fn aspect(ctx: &mut Ctx) -> f32 {
  // TODO memoize in state on window size change
  window_width(ctx) / window_height(ctx)
}

/// half the current window width in device pixels

pub fn window_width_half(_: &Ctx) -> f32 {
  // TODO memoize in state on window size change
  (sapp_width() as f32) / 2.0
}

/// half the current window height in device pixels

pub fn window_height_half(_: &Ctx) -> f32 {
  // TODO memoize in state on window size change
  (sapp_height() as f32) / 2.0
}

/// returns a world position corresponding to the given window position. This is a bit faster
/// than the corresponding

pub fn window_to_world_2d(ctx: &Ctx, p: Vec2) -> Vec2 {
  let window_width = window_width(ctx) as f32;

  let window_height = window_height(ctx) as f32;

  let world_pos = vec2(p.x() - window_width / 2.0, window_height / 2.0 - p.y());

  world_pos
}

// ----------------------------------------------------------------------------
// GRAPHICS SETUP

/// configures kit to use the default 2d projection for rendering.
/// In this projection, 1 world unit is equal to 1 device pixel.
/// However, we set the world origin to be the center of the screen and y points up.
///
/// Should be called every frame if the window size is changeable.

pub fn default_projection_2d(ctx: &mut Ctx) {
  let half_w = window_width_half(ctx);

  let half_h = window_height_half(ctx);

  let camera_pos = vec3(0.0, 0.0, 6.0);

  ctx.gfx.proj = Mat4::orthographic_rh_gl(-half_w, half_w, -half_h, half_h, f32::MIN, f32::MAX);

  ctx.gfx.view = Mat4::look_at_rh(camera_pos, Vec3::zero(), Vec3::unit_y());
}

// ----------------------------------------------------------------------------
// IMAGE LOADING

// TODO unload textures?

// ----------------------------------------------------------------------------
// asset loading

/// Loads an image into memory. Returns info about the image, including width,
/// height, and an id for setting the image for use in draw calls.

pub fn load_img(ctx: &mut Ctx, filename: &str) -> Texture {
  let id = ctx.gfx.images.count;

  ctx.gfx.images.count += 1;

  // TODO get the true path using the base... is this needed or does the Rust std lib do this for me?
  let path = application_root_dir().join(filename);

  // TODO when I switch to OpenGL, I may just want to use a surface to load pixel data
  let img = image::open(path.clone());

  let (img_ptr, w, h) = match img {
    Err(e) => {
      println!("Error loading image at {:?}: {}", path, e);

      let img_fallback: Vec<u8> = vec![0];

      (img_fallback.as_ptr(), 1, 1)
    }
    Ok(img) => {
      let img = img.into_rgba();

      let (w, h) = img.dimensions();

      let img_data = img.into_raw();

      let img_ptr: *const u8 = img_data.as_ptr();

      (img_ptr, w, h)
    }
  };

  let width = w as i32;

  let height = h as i32;

  // let num_channels = num_channels as i32;
  let size: i32 = width * height * 8 /* bytes per pixel */;

  let e = sg_make_image(
    Some(&[(img_ptr, size)]),
    &SgImageDesc {
      width,
      height,
      pixel_format: SgPixelFormat::RGBA8,
      min_filter: SgFilter::Nearest,
      mag_filter: SgFilter::Nearest,
      wrap_u: SgWrap::ClampToEdge,
      wrap_v: SgWrap::ClampToEdge,
      ..Default::default()
    },
  );

  ctx.gfx.images.e[id] = Image { e, w, h };

  Texture { id, w, h }
}

/// Helper for shape construction. Most of our primitives take a standard mvp
/// matrix as a uniform which is used for camera position, so they usually implement
/// this standard uniform block in their shaders.

pub(crate) fn std_uniform_block<'a>() -> SgShaderUniformBlockDesc<'a> {
  SgShaderUniformBlockDesc {
    size: size_of::<Mat4>() as i32,
    uniforms: vec![SgShaderUniformDesc {
      name: "projection",
      uniform_type: SgUniformType::Mat4,
      array_count: 0,
    }],
  }
}

// ----------------------------------------------------------------------------
// LIFECYCLE

/// lifecycle function for initial setup and sensible defaults. Needs to be
/// run *after* window initialization.

pub fn init(ctx: &mut Ctx) {
  sg_setup(&SgDesc {
    ..Default::default()
  });

  ctx.gfx.proj = Mat4::identity();

  ctx.gfx.view = Mat4::identity();

  ctx.gfx.pass_action = SgPassAction {
    colors: vec![SgColorAttachmentAction {
      action: SgAction::Clear,
      val: [0.2, 0.2, 0.2, 1.0],
    }],
    ..Default::default()
  };

  // initialize each primitive shape's memory for draw commands
  // shader, and pipeline
  mesh::init(ctx);

  line::init(ctx);

  point::init(ctx);

  quad::init(ctx);
}

/// Lifecycle function for processing all draw calls collected for a single
/// frame of rendering. Clears all calls when done to prepare for the next frame.

pub fn present(ctx: &mut Ctx) {
  sg_begin_default_pass(&ctx.gfx.pass_action, sapp_width(), sapp_height());

  mesh::present(ctx);
  quad::present(ctx);
  point::present(ctx);
  line::present(ctx);

  // clear all draw calls
  ctx.gfx.quads.count = 0;
  ctx.gfx.points.count = 0;
  ctx.gfx.lines.count = 0;
  ctx.gfx.mesh.count = 0;

  sg_end_pass();
  sg_commit();
}
