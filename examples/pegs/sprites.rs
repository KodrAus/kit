use crate::types::*;
use kit::*;

mod peg_beige;
mod peg_blue;
mod peg_green;
mod peg_pink;
mod peg_yellow;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  peg_beige::init(ctx, state);
  peg_blue::init(ctx, state);
  peg_green::init(ctx, state);
  peg_pink::init(ctx, state);
  peg_yellow::init(ctx, state);
}
