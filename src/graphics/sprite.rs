use crate::graphics::quad::draw_quad;
use crate::math::*;
use crate::*;

/**
 * This draw command is a special alias for `draw_quad` which processes
 * a spritesheet (Texture) and draws a segment of it (TextureFrame)
 * onto the quad.
 */
pub fn draw_sprite(
    ctx: &mut Ctx,
    pos: V2,
    texture: Texture,
    frame: TextureFrame,
    scale: f32,
    z: f32,
) {
    if scale == 0.0 {
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
        QuadVert::new(min.x, min.y, z, uv_l, 1.0),
        QuadVert::new(max.x, min.y, z, uv_r, 1.0),
        QuadVert::new(min.x, max.y, z, uv_l, 0.0),
        QuadVert::new(max.x, max.y, z, uv_r, 0.0),
    ];

    let mut transform = M4::diag(1.0);

    // translation
    transform.e[3][0] = pos.x;
    transform.e[3][1] = pos.y;
    // quad.model.e[3][2] = sprite.pos.z;

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
