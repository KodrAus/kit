use crate::types::*;
use crate::utils::*;
use kit::*;

pub fn update(ctx: &Ctx, state: &mut State, from_pos: Coords) {
  let click = ctx.input.mouse.left.down > 0;
  if click {
    let click_pos = window_to_world(ctx, ctx.input.mouse.pos);
    let mut clicked_tile = None;
    for pos in state.board.iterator() {
      let tile_bounds = tile_bounds(ctx, pos);
      if test_point_v_circle(click_pos, tile_bounds) {
        clicked_tile = Some(pos);
      }
    }
    match clicked_tile {
      None => {}
      Some(to_pos) => {
        let valid = valid_move(&state.board, from_pos, to_pos);
        match valid {
          Some(over_pos) => {
            let from_i = state.board.get(from_pos).unwrap();
            let over_i = state.board.get(over_pos).unwrap();
            peg_jump(state, from_i, from_pos, to_pos);
            peg_die(state, over_i, over_pos);
            // note we'll actually change the board contents in the jump phase,
            // because we need to let the animations run their course before we
            // remove stuff from the board
            state.phase = Phase::Jump;
          }
          None => {}
        }
      }
    }
  }
}
