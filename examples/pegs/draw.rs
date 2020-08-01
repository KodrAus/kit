mod debug;
mod gui;
mod pegs;
mod tiles;

use crate::types::PegType;
use crate::types::State;
use kit::*;

pub fn draw(ctx: &mut Ctx, state: &State) {
  default_projection_2d(ctx);
  tiles::draw(ctx, state);
  pegs::draw(ctx, state);
  gui::draw(ctx, state);
  // debug::draw(ctx, state);
}
