use crate::constants::*;
use crate::types::Coords;
use crate::types::State;
use crate::utils::board_to_screen_position;
use kit::*;

const TILE_WIDTH: u32 = 92 * 2;
const TILE_HEIGHT: u32 = 126 * 2;
const SCALE: f32 = BOARD_CELL_WIDTH_PX as f32 / TILE_WIDTH as f32;

fn pivot_px() -> Pivot {
  Pivot::Px(0.5 * TILE_WIDTH as f32, 0.625 * TILE_HEIGHT as f32)
}

pub fn draw(ctx: &mut Ctx, state: &State) {
  for pos in state.board.iterator() {
    draw_tile(ctx, state, pos);
  }
}

fn draw_tile(ctx: &mut Ctx, state: &State, pos: Coords) {
  let screen_pos = board_to_screen_position(pos);
  let texture = state.assets.tile.unwrap();
  let texture_id = texture.id;
  kit::draw_image(
    ctx,
    texture_id,
    pivot_px(),
    Transform2d {
      pos: screen_pos,
      scale: Vec2::splat(SCALE),
      ..Default::default()
    },
  );
}
