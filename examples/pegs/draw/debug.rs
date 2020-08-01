use crate::types::*;
use crate::utils::*;
use kit::*;

pub fn draw(ctx: &mut Ctx, state: &State) {
  draw_debug_mouse_pos(ctx, state);
  draw_debug_peg_hit_areas(ctx, state);
  draw_debug_tile_hit_areas(ctx, state);
  draw_debug_reset_btn(ctx, state);
}

fn draw_debug_reset_btn(ctx: &mut Ctx, state: &State) {
  let rect = reset_btn_rect_world(ctx, state);
  draw_rect(ctx, rect, red());
}

/// draws a circle around the mouse position in world space
/// to help debug logic that converts window position to
/// world position.
fn draw_debug_mouse_pos(ctx: &mut Ctx, state: &State) {
  let center = state.mouse_pos;
  draw_circ(ctx, Circle { center, r: 10.0 }, red());
}

/// draws hit bounderies for pegs and highlights the "hovered"
/// peg boundary.
fn draw_debug_peg_hit_areas(ctx: &mut Ctx, state: &State) {
  let over_peg = state.over_peg;

  for pos in state.board.iterator() {
    let peg_i = state.board.get(pos);
    let color = if (over_peg.is_some() && over_peg.unwrap() == (pos)) {
      white()
    } else {
      red()
    };
    match peg_i {
      Some(peg_i) => {
        let peg_type = state.pegs.peg_type[peg_i];
        draw_peg_bounds(ctx, pos, peg_type, color);
      }
      None => {}
    }
  }
}

/// draws hit bounderies for tiles and highlights the "hovered"
/// peg boundary.
fn draw_debug_tile_hit_areas(ctx: &mut Ctx, state: &State) {
  for pos in state.board.iterator() {
    let peg_i = state.board.get(pos);
    let color = white();
    draw_tile_bounds(ctx, pos, color);
  }
}

pub fn draw_peg_bounds(ctx: &mut Ctx, pos: Coords, peg_type: PegType, color: Vec4) {
  // TODO maybe store bounds on state to avoid recalc
  let (head, body, feet) = peg_bounds(ctx, pos, peg_type);
  draw_circ(ctx, head, color);
  draw_circ(ctx, feet, color);
  draw_rect(ctx, body, color);
}

pub fn draw_tile_bounds(ctx: &mut Ctx, pos: Coords, color: Vec4) {
  // TODO maybe store bounds on state to avoid recalc
  let c = tile_bounds(ctx, pos);
  draw_circ(ctx, c, color);
}
