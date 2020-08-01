use crate::types::*;
use kit::*;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  let id = state.assets.peg_yellow.unwrap().id;
  state.sprites.peg_yellow = PegSheet {
    hurt: sprite(ctx, id, 1, 1, 69, 81, Pivot::Px(43.0, 0.0)),
    jump: sprite(ctx, id, 1, 153, 67, 83, Pivot::Px(30.0, 0.0)),
    front: sprite(ctx, id, 1, 238, 66, 82, Pivot::Px(32.0, 0.0)),
    lean: sprite(ctx, id, 1, 322, 66, 82, Pivot::Px(28.0, 0.0)),
    duck: sprite(ctx, id, 1, 84, 68, 67, Pivot::Px(27.0, 0.0)),
  };
}
