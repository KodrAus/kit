use crate::constants::*;
use crate::types::*;
use crate::utils::*;
use kit::*;

pub fn update(ctx: &Ctx, state: &mut State) {
  // all animation must be stable for us to move on to the next phase
  for i in 0..MAX_PEGS {
    match state.pegs.state[i] {
      PegState::Jump(..) | PegState::Dying(..) => return,
      _ => {}
    }
  }
  state.phase = Phase::Picking;
}
