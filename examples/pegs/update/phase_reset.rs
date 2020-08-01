use crate::constants::*;
use crate::types::*;
use crate::utils::*;
use kit::*;

pub fn update(ctx: &Ctx, state: &mut State) {
  if state.reset_spin_animation < RESET_SPIN_DURATION {
    state.reset_spin_animation += 1;
  }
  if state.board.count() == 0 {
    populate(state);
    state.phase = Phase::Picking;
  }
}
