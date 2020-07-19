use crate::graphics::quad::draw_quad;
use crate::math::*;
use crate::*;

/// This draw command is a special alias for `draw_quad` which processes
/// a spritesheet (Texture) and draws a segment of it (TextureFrame)
/// onto the quad.
///
/// Note: Future versions may iterate on the TextureFrame format so that some
/// of this math can be pre-baked (for example, a TextureFrame's boundaries
/// could already be expressed in UVs without needing extra calculation here).
pub fn draw_sprite(ctx: &mut Ctx, pos: V2, texture: Texture, frame: TextureFrame, scale: f32) {
    if near_zero(scale) {
        // TODO maybe near_zero?
        return;
    }

    // TODO precalculate UVs based on metadata rather than redoing this for
    // each quad? Or I could push it into the shader where it would be highly
    // parallelized.
    let sheet_w = texture.w;
    let uv_l: f32 = frame.x as f32 / sheet_w as f32;
    let uv_r: f32 = (frame.x + frame.w) as f32 / sheet_w as f32;
    let min = V2::ZERO - frame.offset;
    let max = min + v2(frame.w as f32, frame.h as f32);

    let corners = [
        QuadVert::new(min.x, min.y, 0.0, uv_l, 1.0),
        QuadVert::new(max.x, min.y, 0.0, uv_r, 1.0),
        QuadVert::new(min.x, max.y, 0.0, uv_l, 0.0),
        QuadVert::new(max.x, max.y, 0.0, uv_r, 0.0),
    ];

    let mut transform = M4::diag(1.0);

    // translation
    transform.e[3][0] = pos.x;
    transform.e[3][1] = pos.y;
    // transform.e[3][2] = pos.z; // no z for sprites; they're 2D

    // scale
    transform.e[0][0] = scale;
    transform.e[1][1] = scale;
    transform.e[2][2] = scale;

    draw_quad(
        ctx,
        DrawQuad {
            texture_id: texture.id,
            corners,
            transform,
        },
    );
}
