use crate::board::Board;
use crate::constants::*;
use crate::types::*;
use kit::*;

pub fn middle_pos(a: Coords, b: Coords) -> Coords {
  let delta_x = a.x - b.x;
  let delta_y = a.y - b.y;
  return coords(delta_x / 2 + b.x, delta_y / 2 + b.y);
}

pub fn valid_pos(pos: Coords) -> bool {
  pos.x <= pos.y && pos.y < BOARD_SIZE && pos.y >= 0 && pos.x >= 0
}

pub fn valid_dir(board: &Board, from: Coords, dir: Coords) -> Option<Coords> {
  let over = from + dir;
  let to = over + dir;
  if valid_pos(to) && board.get(to).is_none() && board.get(over).is_some() {
    Some(over)
  } else {
    None
  }
}

pub fn valid_move(board: &Board, from: Coords, to: Coords) -> Option<Coords> {
  let delta = to - from;
  for i in 0..MOVE_DIRECTIONS.len() {
    let dir = MOVE_DIRECTIONS[i];
    if dir * 2 != delta {
      continue;
    }
    let result = valid_dir(board, from, dir);
    if result.is_some() {
      return result;
    }
  }
  None
}

const MOVE_DIRECTIONS: [Coords; 6] = [
  Coords { x: 0, y: 1 },
  Coords { x: 0, y: -1 },
  Coords { x: -1, y: -1 },
  Coords { x: -1, y: 0 },
  Coords { x: 1, y: 1 },
  Coords { x: 1, y: 0 },
];

pub fn valid_moves(board: &Board, from: Coords) -> bool {
  // make sure there's a peg at the coordinates
  if board.get(from).is_none() {
    false
  } else {
    for i in 0..MOVE_DIRECTIONS.len() {
      let dir = MOVE_DIRECTIONS[i];
      if valid_dir(board, from, dir).is_some() {
        return true;
      }
    }
    return false;
  }
}

/// returns a visual offset for the given row in pixel-space
/// so that drawing the board creates a pyramid shape which
/// is centered on screen
fn board_to_screen_position_offset_x(y: i32) -> f32 {
  let y = y as f32;
  -y * BOARD_CELL_WIDTH_PX_HALF
}

pub fn board_to_screen_position(pos: Coords) -> Vec2 {
  let offset_x = board_to_screen_position_offset_x(pos.y);
  let offset_y = BOARD_SIZE as f32 * BOARD_CELL_HEIGHT_PX_HALF - 60.0;
  vec2(
    pos.x as f32 * BOARD_CELL_WIDTH_PX + offset_x,
    -pos.y as f32 * BOARD_CELL_HEIGHT_PX + offset_y,
  )
}

pub fn screen_to_board_position(screen_pos: Vec2) -> Coords {
  // let y = (screen_pos.y / BOARD_ROW_HEIGHT) as i32;
  // let x = ((screen_pos.x - x_offset(y)) / BOARD_GRID_SIZE as f32) as i32;
  let x = 0;
  let y = 0; // TODO
  return coords(x, y);
}

pub fn test_peg(ctx: &Ctx, board_pos: Coords, peg_type: PegType, p: Vec2) -> bool {
  let (head, body, feet) = peg_bounds(ctx, board_pos, peg_type);
  test_point_v_circle(p, head) || test_point_v_circle(p, feet) || test_point_v_aabb(p, body)
}

pub fn peg_bounds(ctx: &Ctx, pos: Coords, peg_type: PegType) -> (Circle, Rect, Circle) {
  let r = 40.0;
  let bottom_offset = 14.0;
  let height = match peg_type {
    PegType::Beige => 60.0 - bottom_offset,
    PegType::Pink => 60.0 - bottom_offset,
    PegType::Yellow => 50.0 - bottom_offset,
    PegType::Green => 60.0 - bottom_offset,
    PegType::Blue => 60.0 - bottom_offset,
  };
  let base = board_to_screen_position(pos) + vec2(0.0, bottom_offset);
  let top = base + vec2(0.0, height);
  let center = top;
  // TODO would be nice if we implemented positional add
  // for circle shapes
  let head = Circle { r, center: top };
  let feet = Circle { r, center: base };
  let body = Rect {
    min_x: base.x() - r,
    max_x: base.x() + r,
    min_y: base.y(),
    max_y: top.y(),
  };
  // TODO Capsule hit shape? Hit shape groups?
  (head, body, feet)
}

pub fn tile_bounds(ctx: &Ctx, pos: Coords) -> Circle {
  Circle {
    r: 40.0,
    center: board_to_screen_position(pos),
  }
}

pub fn window_center_px(ctx: &Ctx) -> Vec2 {
  vec2(window_width(ctx) as f32, window_height(ctx) as f32) / 2.0
}

pub fn window_to_world(ctx: &Ctx, p: Vec2) -> Vec2 {
  let window_width = window_width(ctx) as f32;
  let window_height = window_height(ctx) as f32;
  let world_pos = vec2(p.x() - window_width / 2.0, window_height / 2.0 - p.y());

  // TODO mouse coords are y-down and relative to the upper
  // left corner of the screen - kit's default 2D rendering
  // view uses y-up and the origin is at the center of the
  // window. This makes converting mouse pos to world pos
  // for hit testing a bit hairy (see below for a general
  // approach). Could I do GPU-based hit testing? How can
  // I make hit testing simpler? Should the default coord
  // system use the window coordinates (y-down, upper-left
  // origin)?
  // let world_to_window_vp = ctx.gl.view * ctx.gl.proj;
  // let window_to_world_vp = world_to_window_vp.inverse();
  // let window_normalized_pos = vec4(
  //   lerpf(-1.0, 1.0, p.x() / window_width),
  //   lerpf(1.0, -1.0, p.y() / window_height),
  //   0.0,
  //   1.0,
  // );
  // let world_pos = window_to_world_vp * window_normalized_pos;
  // let world_pos = vec2(world_pos.x(), world_pos.y());

  world_pos
}

pub fn grounded(state: &State, peg_i: usize) -> bool {
  let z = state.pegs.z[peg_i];
  let z_vel = state.pegs.z_vel[peg_i];
  near_zero(z) && near_zero(z_vel)
}

/// fills the game board with pegs; only call when the board is empty!
pub fn populate(state: &mut State) {
  let empty = Coords { x: 1, y: 2 };
  let mut peg_i = 0;
  for pos in state.board.iterator() {
    if pos == empty {
      continue;
    }
    let peg_type: PegType = rand::random();
    state.pegs.peg_type[peg_i] = peg_type;
    state.pegs.z[peg_i] = rand::random::<f32>() * DROP_HEIGHT_VARIANCE + DROP_HEIGHT_MIN;
    state.board.set(pos, Some(peg_i));
    peg_i += 1;
  }
}

pub fn peg_idle(state: &mut State, i: usize) {
  state.pegs.state[i] = PegState::Idle;
  state.pegs.animation[i] = 0;
  state.pegs.lean[i] = 0.0;
}

pub fn peg_excited(state: &mut State, i: usize, pos: Coords) {
  state.phase = Phase::Excited(pos);
  state.pegs.state[i] = PegState::Excited;
  state.pegs.animation[i] = 0;
  state.pegs.lean[i] = 1.0;
}

pub fn peg_buzz(state: &mut State, i: usize) {
  match state.pegs.state[i] {
    PegState::Buzz => {}
    _ => {
      state.pegs.state[i] = PegState::Buzz;
      state.pegs.animation[i] = 0;
      state.pegs.lean[i] = 0.0;
      // TODO audio
    }
  }
}

pub fn peg_jump(state: &mut State, i: usize, from: Coords, to: Coords) {
  // TODO maybe embed the animation counter into the state enum?
  state.pegs.state[i] = PegState::Jump(from, to);
  state.pegs.animation[i] = 0;
}

pub fn peg_die(state: &mut State, i: usize, pos: Coords) {
  state.pegs.state[i] = PegState::Dying(pos);
  state.pegs.animation[i] = 0;
}

pub fn peg_dead(state: &mut State, i: usize) {
  state.pegs.state[i] = PegState::Dead;
  state.pegs.animation[i] = 0;
}

pub fn game_over_message(state: State) -> &'static str {
  match state.board.count() {
    0 => "You're a Genius",
    1 => "You're Pretty Smart",
    2 => "Just Plain Dumb",
    3 => "Just Plain Eg-no-ra-moose",
    _ => "",
  }
}

pub fn is_game_over(state: &State) -> bool {
  !state
    .board
    .iterator()
    .any(|pos| valid_moves(&state.board, pos))
}

pub fn reset_btn_img(state: &State) -> Texture {
  state.assets.reset.unwrap()
}

const RESET_BTN_PAD: f32 = 50.0;

pub fn reset_btn_pos(ctx: &Ctx, state: &State) -> Vec2 {
  vec2(
    RESET_BTN_PAD - window_width_half(ctx),
    window_height_half(ctx) - RESET_BTN_PAD,
  )
}

pub fn reset_btn_rect(state: &State) -> Rect {
  let img = reset_btn_img(state);
  Rect::centered(img.w as f32, img.h as f32)
}

pub fn reset_btn_rect_world(ctx: &Ctx, state: &State) -> Rect {
  let rect = reset_btn_rect(state);
  let pos = reset_btn_pos(ctx, state);
  rect + pos
}
