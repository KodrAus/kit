use crate::types::*;
use crate::utils::*;
use kit::*;

pub fn update(ctx: &Ctx, state: &mut State) {
  // clicks aren't always registering, but only when not plugged in? Debug this!
  let click = ctx.input.mouse.left.down > 0;
  if click {
    let picked_peg = state.over_peg;
    match picked_peg {
      None => {}
      Some(pos) => {
        let peg_i = state.board.get(pos).unwrap(); // picked position must have a peg!
        let valid = valid_moves(&state.board, pos);
        if valid {
          peg_excited(state, peg_i, pos);
        } else {
          peg_buzz(state, peg_i);
        }
      }
    }
  }
}
