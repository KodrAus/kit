use crate::geometry::*;
use crate::graphics::line::draw_line;
use crate::math::*;
use crate::Ctx;
use glam::*;

pub fn draw_rect(ctx: &mut Ctx, aabb: Rect, color: Vec4) {
    let a = vec2(aabb.max_x, aabb.max_y);
    let b = vec2(aabb.max_x, aabb.min_y);
    let c = vec2(aabb.min_x, aabb.min_y);
    let d = vec2(aabb.min_x, aabb.max_y);
    draw_line(ctx, a.extend(0.0), b.extend(0.0), color);
    draw_line(ctx, b.extend(0.0), c.extend(0.0), color);
    draw_line(ctx, c.extend(0.0), d.extend(0.0), color);
    draw_line(ctx, d.extend(0.0), a.extend(0.0), color);
}
