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
use std::path::Path;

const BYTES_POINTS: usize = size_of::<DrawPoint>() * MAX_POINTS;
const BYTES_LINES: usize = size_of::<DrawLine>() * MAX_LINES;

// struct MeshVsParams {
//     model: Mat4,
//     view_proj: Mat4,
// }

pub fn load_img(ctx: &mut Ctx, filename: &str) -> Texture {
    // TODO get the true path using the base... is this needed or does the Rust std lib do this for me?
    let path = Path::new(filename);

    // TODO when I switch to OpenGL, I may just want to use a surface to load pixel data
    let img = image::open(path).unwrap();
    gl_register_img(ctx, img)
}

///////////////////////////////////////////////////////////////////////////////
// specialized draw calls
// ----------------------
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

/// returns the current aspect ratio of the application window
pub fn aspect(_: &mut Ctx) -> f32 {
    (sapp_width() as f32) / (sapp_height() as f32)
}

/// returns the current window width
pub fn window_width(_: &Ctx) -> u32 {
    (sapp_width() as u32)
}

pub fn window_height(_: &Ctx) -> u32 {
    (sapp_height() as u32)
}

pub fn default_projection(ctx: &mut Ctx) {
    let half_w = window_width(ctx) as f32 / 2.0;
    let half_h = window_height(ctx) as f32 / 2.0;
    let camera_pos = vec3(0.0, 0.0, 6.0);
    ctx.gl.proj = Mat4::orthographic_rh_gl(-half_w, half_w, -half_h, half_h, -500.0, 500.0);
    ctx.gl.view = Mat4::look_at_rh(camera_pos, Vec3::zero(), Vec3::unit_y());
}

// TODO unload textures?

/// internal function for registering an image as a texture in graphics memory
fn gl_register_img(ctx: &mut Ctx, img: DynamicImage) -> Texture {
    let id = ctx.gl.images.count;
    ctx.gl.images.count += 1;

    let img = img.into_rgba();
    let (w, h) = img.dimensions();
    let width = w as i32;
    let height = h as i32;
    let img_data = img.into_raw();
    let img_ptr: *const u8 = img_data.as_ptr();

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
    ctx.gl.images.e[id] = Image { e, w, h };

    return Texture { id, w, h };
}

/// Helper for shape construction. Most of our primitives take a standard mvp
/// matrix as a uniform which is used for camera position, so they usually implement
/// this standard uniform block in their shaders.
fn std_uniform_block<'a>() -> SgShaderUniformBlockDesc<'a> {
    SgShaderUniformBlockDesc {
        size: size_of::<Mat4>() as i32,
        uniforms: vec![SgShaderUniformDesc {
            name: "projection",
            uniform_type: SgUniformType::Mat4,
            array_count: 0,
        }],
    }
}

/// lifecycle function for initial setup and sensible defaults. Needs to be
/// run *after* window initialization.
pub fn init(ctx: &mut Ctx) {
    sg_setup(&SgDesc {
        ..Default::default()
    });

    ctx.gl.proj = Mat4::identity();
    ctx.gl.view = Mat4::identity();

    ctx.gl.pass_action = SgPassAction {
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
    sg_begin_default_pass(&ctx.gl.pass_action, sapp_width(), sapp_height());

    mesh::present(ctx);
    quad::present(ctx);
    point::present(ctx);
    line::present(ctx);

    // clear all draw calls
    ctx.gl.quads.count = 0;
    ctx.gl.points.count = 0;
    ctx.gl.lines.count = 0;
    ctx.gl.mesh.count = 0;

    sg_end_pass();
    sg_commit();
}
