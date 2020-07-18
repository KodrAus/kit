use crate::geometry::*;
use crate::graphics::line::draw_line;
use crate::math::*;
use crate::Ctx;

pub fn draw_rect(ctx: &mut Ctx, aabb: Rect, color: V4) {
    let a = v2(aabb.max_x, aabb.max_y);
    let b = v2(aabb.max_x, aabb.min_y);
    let c = v2(aabb.min_x, aabb.min_y);
    let d = v2(aabb.min_x, aabb.max_y);
    draw_line(ctx, a.into(), b.into(), color);
    draw_line(ctx, b.into(), c.into(), color);
    draw_line(ctx, c.into(), d.into(), color);
    draw_line(ctx, d.into(), a.into(), color);
}
