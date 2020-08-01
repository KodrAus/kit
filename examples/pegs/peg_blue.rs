use crate::types::*;
use kit::*;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  let id = state.assets.peg_blue.unwrap().id;
  state.sprites.peg_blue = PegSheet {
    jump: sprite(ctx, id, 1, 1, 66, 93, Pivot::Px(31.0, 0.0)),
    hurt: sprite(ctx, id, 1, 96, 67, 92, Pivot::Px(41.0, 0.0)),
    front: sprite(ctx, id, 1, 190, 66, 92, Pivot::Px(32.0, 0.0)),
    lean: sprite(ctx, id, 1, 284, 66, 92, Pivot::Px(29.0, 0.0)),
    duck: sprite(ctx, id, 1, 377, 67, 72, Pivot::Px(28.0, 1.0)),
  };
}
