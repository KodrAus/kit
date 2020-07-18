use crate::geometry::*;
use crate::graphics::line::draw_line;
use crate::math::*;
use crate::*;

const CIRC_SEGMENTS: usize = 16;
const SEGMENT_ARC: f32 = TAU / CIRC_SEGMENTS as f32;

// TODO create a shader for circles so we don't have to compose them of points
pub fn draw_circ(ctx: &mut Ctx, c: Circle, color: V4) {
    // draw_point(core, c.center, color);
    let mut points: [V2; CIRC_SEGMENTS] = [V2::ZERO; CIRC_SEGMENTS];
    for i in 0..CIRC_SEGMENTS {
        let arc = SEGMENT_ARC * i as f32;
        let x = f32::cos(arc);
        let y = f32::sin(arc);
        points[i] = v2(x, y);
    }

    let mut c_points: [V2; CIRC_SEGMENTS] = [V2::ZERO; CIRC_SEGMENTS];
    for i in 0..CIRC_SEGMENTS {
        c_points[i] = c.center + points[i] * c.r;
    }

    // could pre-calc and/or optimize this via reflection
    for i in 0..(CIRC_SEGMENTS - 1) {
        draw_line(ctx, c_points[i].into(), c_points[i + 1].into(), color);
    }
    draw_line(
        ctx,
        c_points[0].into(),
        c_points[CIRC_SEGMENTS - 1].into(),
        color,
    );
}
