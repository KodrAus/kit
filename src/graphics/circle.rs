use crate::geometry::*;
use crate::graphics::line::draw_line;
use crate::math::*;
use crate::*;

const CIRC_SEGMENTS: usize = 16;
const SEGMENT_ARC: f32 = TAU / CIRC_SEGMENTS as f32;

// TODO this is just a special mesh. Use meshes.
pub fn draw_circ(ctx: &mut Ctx, c: Circle, color: Vec4) {
  // draw_point(core, c.center, color);
  let mut points: [Vec2; CIRC_SEGMENTS] = [Vec2::zero(); CIRC_SEGMENTS];

  for i in 0..CIRC_SEGMENTS {
    let arc = SEGMENT_ARC * i as f32;
    let x = f32::cos(arc);
    let y = f32::sin(arc);
    points[i] = vec2(x, y);
  }

  let mut c_points: [Vec2; CIRC_SEGMENTS] = [Vec2::zero(); CIRC_SEGMENTS];
  for i in 0..CIRC_SEGMENTS {
    c_points[i] = c.center + points[i] * c.r;
  }

  // could pre-calc and/or optimize this via reflection
  for i in 0..(CIRC_SEGMENTS - 1) {
    // TODO a lot of repetition here
    draw_line(
      ctx,
      c_points[i].extend(0.0).into(),
      c_points[i + 1].extend(0.0).into(),
      color,
    );
  }

  draw_line(
    ctx,
    c_points[0].extend(0.0).into(),
    c_points[CIRC_SEGMENTS - 1].extend(0.0).into(),
    color,
  );
}
