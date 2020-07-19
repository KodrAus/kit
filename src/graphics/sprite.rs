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
pub fn draw_sprite(ctx: &mut Ctx, sprite: Sprite, pos: V2, scale: f32) {
    let corners = sprite.corners;
    let img_id = sprite.img_id;
    let transform = sprite_transform(pos, scale);
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
pub fn sprite_transform(pos: V2, scale: f32) -> M4 {
    let mut transform = M4::IDENTITY;

    // translation
    transform.e[3][0] = pos.x;
    transform.e[3][1] = pos.y;
    // transform.e[3][2] = pos.z; // no z for sprites; they're 2D

    // scale
    transform.e[0][0] = scale;
    transform.e[1][1] = scale;
    transform.e[2][2] = scale;

    transform
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
pub fn draw_image(ctx: &mut Ctx, img_id: usize, pos: V2, scale: f32, pivot_px: V2) {
    let w = ctx.gl.images.e[img_id].w as f32;
    let h = ctx.gl.images.e[img_id].h as f32;
    let l = -pivot_px.x;
    let d = -pivot_px.y;
    let r = l + w;
    let u = d + h;
    let z = 0.0;
    let corners = [
        QuadVert::new(l, d, z, 0.0, 1.0),
        QuadVert::new(r, d, z, 1.0, 1.0),
        QuadVert::new(l, u, z, 0.0, 0.0),
        QuadVert::new(r, u, z, 1.0, 0.0),
    ];
    let sprite = Sprite { img_id, corners };
    draw_sprite(ctx, sprite, pos, scale);
}
