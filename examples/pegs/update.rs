use crate::constants::*;
use crate::types::*;
use crate::utils::*;
use kit::*;

mod phase_excited;
mod phase_jump;
mod phase_picking;
mod phase_reset;

pub fn update(ctx: &Ctx, state: &mut State) {
  // TODO gamepad input
  // TODO animate peg hover state
  // TODO highlight valid moves

  update_mouse_pos(ctx, state);
  update_over_peg(ctx, state);
  update_peg_animations(ctx, state);
  update_peg_z(ctx, state);
  update_phase(ctx, state);
  update_reset(ctx, state);
}

fn update_reset(ctx: &Ctx, state: &mut State) {
  let click = ctx.input.mouse.left.down > 0;
  if !click {
    return;
  }
  let rect = reset_btn_rect_world(ctx, state);
  let btn_click = test_point_v_aabb(state.mouse_pos, rect);
  if btn_click {
    state.reset_spin_animation = 0;
    state.phase = Phase::Reset;
    for pos in state.board.iterator() {
      let peg_i = state.board.get(pos);
      match peg_i {
        Some(peg_i) => {
          peg_die(state, peg_i, pos);
        }
        None => {}
      }
    }
  }
}

/// calculates the mouse's position in world and stores it on
/// the state so we only have to do this once per frame
fn update_mouse_pos(ctx: &Ctx, state: &mut State) {
  let mouse_pos = ctx.input.mouse.pos;
  state.mouse_pos = window_to_world(ctx, mouse_pos);
}

/// calculates whether the current mouse world position is over
/// a peg and if so, which peg position it is over
fn update_over_peg(ctx: &Ctx, state: &mut State) {
  let mouse_pos = state.mouse_pos;
  state.over_peg = None;
  for pos in state.board.iterator() {
    let peg_i = state.board.get(pos);
    match peg_i {
      Some(peg_i) => {
        let peg_type = state.pegs.peg_type[peg_i];
        if test_peg(ctx, pos, peg_type, mouse_pos) {
          state.over_peg = Some(pos);
        }
      }
      None => {}
    }
  }
}

fn update_phase(ctx: &Ctx, state: &mut State) {
  match state.phase {
    Phase::Picking => phase_picking::update(ctx, state),
    Phase::Excited(pos) => phase_excited::update(ctx, state, pos),
    Phase::Jump => phase_jump::update(ctx, state),
    Phase::Reset => phase_reset::update(ctx, state),
  }
}

fn update_peg_z(ctx: &Ctx, state: &mut State) {
  for i in 0..MAX_PEGS {
    let peg_state = state.pegs.state[i];
    match peg_state {
      PegState::Jump(..) => {
        let percent = state.pegs.animation[i] as f32 / JUMP_DURATION as f32;
        state.pegs.z[i] = (percent * PI).sin() * JUMP_HEIGHT;
      }
      _ => {
        // TODO replace with easing?
        let z = state.pegs.z[i];
        let z_vel = state.pegs.z_vel[i];
        let z_vel = z_vel + DROP_GRAVITY_PER_DT; // add in acceleration
        let mut z_vel = clampf(z_vel, -DROP_TERMINAL_VEL, DROP_TERMINAL_VEL);
        let mut z = z + z_vel;

        if z.abs() < 1.0 && z_vel.abs() < 1.0 {
          // gotta stop sometime
          z = 0.0;
          z_vel = 0.0;
        } else if z < 0.0 {
          // can't be below ground! flip to bounce
          z = -z;
          z_vel = -z_vel * DROP_BOUNCE_DAMPENING; // flip and dampen vel
        }
        state.pegs.z[i] = z;
        state.pegs.z_vel[i] = z_vel;
      }
    }
  }
}

fn update_peg_animations(ctx: &Ctx, state: &mut State) {
  for i in 0..MAX_PEGS {
    let peg_state = state.pegs.state[i];
    let animation = state.pegs.animation[i];

    match peg_state {
      PegState::Buzz => {
        // exit or advance animation
        if (animation >= BUZZ_STATE_DURATION) {
          peg_idle(state, i);
        } else {
          state.pegs.lean[i] = (animation as f32 / BUZZ_STATE_DURATION as f32 * TAU * 2.0).sin();
          state.pegs.animation[i] += 1;
        }
      }
      PegState::Excited => {
        if animation == 0 {
          // hop right
          state.pegs.lean[i] = 1.0;
          state.pegs.z_vel[i] = EXCITED_HOP_POWER;
        } else if animation == EXCITED_HOP_INTERVAL {
          // hop left
          state.pegs.lean[i] = -1.0;
          state.pegs.z_vel[i] = EXCITED_HOP_POWER;
        } else if grounded(state, i) {
          state.pegs.lean[i] = 0.0;
        }
        // reset or advance animation
        if animation >= 2 * EXCITED_HOP_INTERVAL {
          state.pegs.animation[i] = 0;
        } else {
          state.pegs.animation[i] += 1;
        }
      }
      PegState::Jump(from, to) => {
        state.pegs.animation[i] += 1;
        state.pegs.lean[i] = (to.x - from.x) as f32;
        if (state.pegs.animation[i] == JUMP_DURATION / 2) {
          // swap in the middle of the jump so we can use board order layering
          state.board.set(from, None);
          state.board.set(to, Some(i));
        } else if (state.pegs.animation[i] > JUMP_DURATION) {
          peg_idle(state, i);
        }
      }
      PegState::Dying(pos) => {
        state.pegs.animation[i] += 1;
        if (state.pegs.animation[i] > JUMP_DURATION) {
          state.pegs.state[i] = PegState::Dead;
          state.board.set(pos, None);
        } else {
          let progress = state.pegs.animation[i] as f32 / JUMP_DURATION as f32;
          state.pegs.lean[i] = (progress * 15.0).sin();
        }
      }
      _ => {}
    }
  }
}
