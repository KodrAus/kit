use crate::constants::*;
use crate::types::State;
use crate::utils;
use kit::*;

fn reset_btn(ctx: &mut Ctx, state: &State) {
  let img = state.assets.reset.unwrap();
  let pos = utils::reset_btn_pos(ctx, state);
  let pivot = Pivot::Center;
  let rotation = state.reset_spin_animation as f32 / RESET_SPIN_DURATION as f32 * TAU;
  let transform = Transform2d {
    pos,
    rotation,
    ..Default::default()
  };
  draw_image(ctx, img.id, pivot, transform);
}

fn status_msg(ctx: &mut Ctx, state: &State) {
  // TODO show on game over
  // TODO fade in
}

pub fn draw(ctx: &mut Ctx, state: &State) {
  // TODO  GUI (reset_btn & status_msg)
  status_msg(ctx, state);
  reset_btn(ctx, state);
  // TODO mute button
}
