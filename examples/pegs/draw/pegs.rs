use crate::constants::*;
use crate::types::*;
use crate::utils::*;
use kit::*;

const LEAN_THRESHOLD: f32 = 0.5;
// const DUCK_THRESHOLD: f32 = 40.0;
const GROUNDED_THRESHOLD: f32 = 1.0;
const PEG_SHADOW_FADE_DIST: f32 = 50.0;

pub fn draw(ctx: &mut Ctx, state: &State) {
  // TODO death animation
  // TODO move animation
  // TODO excited animation (indicates a peg is selected)

  for pos in state.board.iterator() {
    let i = state.board.get(pos);
    match i {
      None => {}
      Some(i) => {
        let pos = board_to_screen_position(pos);
        draw_shadow(ctx, state, i, pos);
        draw_sprite(ctx, state, i, pos);
      }
    }
  }
}

fn draw_shadow(ctx: &mut Ctx, state: &State, i: usize, pos: Vec2) {
  let peg_type = state.pegs.peg_type[i];
  let peg_state = state.pegs.state[i];
  let peg_animation_frame = state.pegs.animation[i];
  let peg_z = state.pegs.z[i];
  let peg_z_vel = state.pegs.z_vel[i];
  let lean = state.pegs.lean[i];

  let shadow_img_id = state.assets.shadow.unwrap().id;
  let shadow_transform = Transform2d {
    pos,
    scale: Vec2::splat(1.0 - clampf(peg_z / PEG_SHADOW_FADE_DIST, 0.0, 1.0)),
    ..Default::default()
  };
  let shadow_pivot = Pivot::Center;
  kit::draw_image(ctx, shadow_img_id, shadow_pivot, shadow_transform);
}

fn draw_sprite(ctx: &mut Ctx, state: &State, i: usize, pos: Vec2) {
  let peg_type = state.pegs.peg_type[i];
  let peg_state = state.pegs.state[i];
  let z = state.pegs.z[i];
  let lean = state.pegs.lean[i];
  let anim = state.pegs.animation[i];

  let sheet = match peg_type {
    PegType::Beige => &state.sprites.peg_beige,
    PegType::Blue => &state.sprites.peg_blue,
    PegType::Green => &state.sprites.peg_green,
    PegType::Pink => &state.sprites.peg_pink,
    PegType::Yellow => &state.sprites.peg_yellow,
  };

  let z_abs = z.abs(); // distance from ground
  let grounded = grounded(state, i);
  let sprite = if !grounded {
    sheet.jump
  } else if lean.abs() > LEAN_THRESHOLD {
    match peg_state {
      PegState::Dying(..) => sheet.hurt,
      _ => sheet.lean,
    }
  } else {
    sheet.front
  };
  let sprite = lean_sprite(sprite, lean);
  let pos = match peg_state {
    PegState::Jump(from_pos, to_pos) => {
      // interpolate from and to positions based on animation duration
      let percent = anim as f32 / JUMP_DURATION as f32;
      // TODO would this be simpler if I just gave every peg a world position and
      // tweened them every frame toward their current assigned board position?
      let from_pos = board_to_screen_position(from_pos);
      let to_pos = board_to_screen_position(to_pos);
      from_pos.lerp(to_pos, percent)
    }
    _ => pos,
  };
  let pos = vec2(pos.x(), pos.y() + z);
  kit::draw_sprite(
    ctx,
    sprite,
    Transform2d {
      pos,
      ..Default::default()
    },
  );
}

fn lean_sprite(sprite: Sprite, lean: f32) -> Sprite {
  if lean < 0.0 {
    sprite.flip_x()
  } else {
    sprite
  }
}

// const LEAN_THRESHOLD = 0.5;
// const DUCK_THRESHOLD = 40;
// const GROUNDED_THRESHOLD = 5;

//fn sprite_state(state: &State, i: usize) -> PegState {
// TODO
// if (peg_is_grounded(state, i)) {
//   if (Math.abs(self.props.lean) > LEAN_THRESHOLD) {PegState::Lean} else {
//PegState::Idle
//   }
// } else {
//   let absZ = Math.abs(peg_z(state, i));
//   if (absZ < DUCK_THRESHOLD) { PegState::Duck } else {
// PegState::Jump
// }
//}

//   render() {
//     // provide a buffer during the fall to pause in the duck animation
//     let modified_z = Math.max(0, Math.abs(props.z) - DUCK_THRESHOLD);
//     let ground_nearness_factor = lerpf(1.0, 0.0, modified_z / 600.0);
//     let spriteState = self.getSpriteState(this);
//     let { src, pivot, size } = sprites[props.type][spriteState];
//     let flipX = spriteState == 'lean' && props.lean < 0;
//     return (
//       <Group x={props.x} y={props.y}>
//         <Image
//           src={shadowSVG}
//           alpha={props.alive * groundNearnessFactor}
//           /* TODO handle retina better*/
//           scale={props.alive * groundNearnessFactor}
//           width={55}
//           height={34}
//         />
//         <div
//           style={{
//             transform: `scaleX(${flipX ? 1 : -1})`,
//           }}
//         >
//           <Image
//             alpha={props.alive}
//             src={src}
//             onClick={self._onTouch}
//             pivot={pivot}
//             scale={props.alive}
//             width={size.w}
//             height={size.h}
//             y={-modifiedZ} /* reflect to sim a bounce*/
//           />
//         </div>
//       </Group>
//     );
//   }
