use crate::geometry::Rect;
use crate::graphics::quad::draw_quad;
use crate::math::Mat4;
use crate::math::Quat;
use crate::*;

/// This draw command is a special alias for `draw_quad` with some extra
/// utility for treating the associated image like a spritesheet. Define
/// a `Sprite` to describe the zone within the image that will be drawn.
///
/// Use `draw_image` instead if you just want to draw the whole image.
pub fn draw_sprite(ctx: &mut Ctx, sprite: Sprite, transform: Transform2d) {
  let corners = sprite.corners;
  let img_id = sprite.img_id;
  let transform = sprite_transform(transform);
  draw_quad(
    ctx,
    DrawQuad {
      img_id,
      corners,
      transform,
    },
  );
}

/// Creates a transformation matrix based on a 2D position and scaling factor
/// for use in creating quad draw calls that display images
fn sprite_transform(transform: Transform2d) -> Mat4 {
  Mat4::from_scale_rotation_translation(
    transform.scale.extend(1.0),
    Quat::from_rotation_z(transform.rotation),
    transform.pos.extend(0.0),
  )
}

/// Draws a full image on screen at the specified position and scale using a quad.
///
/// A pivot point (here in pixels) in the image will align to the specified position.
/// A pivot point of (0,0) means that the bottom left corner of the image will be
/// placed at the specified position. This is mostly used for scaling *around* the
/// pivot point.
///
/// TODO (wesh) should this convenience method be part of the engine at all?
/// Maybe split out 2D drawing into a separate module.
///
/// TODO more options for pivots... center, for instance
pub fn draw_image(ctx: &mut Ctx, img_id: usize, pivot: Pivot, transform: Transform2d) {
  let w = ctx.gfx.images.e[img_id].w as f32;
  let h = ctx.gfx.images.e[img_id].h as f32;
  let uv = Rect::one();
  let corners = sprite_corners(w, h, uv, pivot);
  let sprite = Sprite { img_id, corners };
  draw_sprite(ctx, sprite, transform);
}

/// Sprite builder. The result can be consumed immediately by `draw_sprite` or stored
/// to avoid recalculating the sprite's corners every frame.
pub fn sprite(ctx: &Ctx, img_id: usize, x: u32, y: u32, w: u32, h: u32, pivot: Pivot) -> Sprite {
  let sheet_w = ctx.gfx.images.e[img_id].w as f32;
  let sheet_h = ctx.gfx.images.e[img_id].h as f32;
  // TODO maybe just use f32 for everything to avoid the casts?
  let x_max = (x + w) as f32;
  let y_max = (y + h) as f32;
  let x = x as f32;
  let y = y as f32;
  let w = w as f32;
  let h = h as f32;
  let uv = Rect {
    min_x: x / sheet_w,
    min_y: y / sheet_h,
    max_x: x_max / sheet_w,
    max_y: y_max / sheet_h,
  };
  let corners = sprite_corners(w, h, uv, pivot);
  Sprite { img_id, corners }
}

/// calculates quad corners for the given sprite dimensions & uv coordinates
fn sprite_corners(w: f32, h: f32, uv: Rect, pivot: Pivot) -> QuadCorners {
  let (min_x, min_y) = match pivot {
    Pivot::Px(x, y) => (-x, -y),
    Pivot::Center => (-w * 0.5, -h * 0.5),
  };
  let max_x = min_x + w;
  let max_y = min_y + h;
  let z = 0.0;
  [
    QuadVert::new(min_x, min_y, z, uv.min_x, uv.max_y),
    QuadVert::new(max_x, min_y, z, uv.max_x, uv.max_y),
    QuadVert::new(min_x, max_y, z, uv.min_x, uv.min_y),
    QuadVert::new(max_x, max_y, z, uv.max_x, uv.min_y),
  ]
}
