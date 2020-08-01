use kit::*;

// grid
pub const BOARD_SIZE: i32 = 5;
pub const BOARD_SIZE_HALF: i32 = BOARD_SIZE / 2;
pub const BOARD_NUM_SPACES: usize = BOARD_SIZE as usize * 3;
pub const MAX_PEGS: usize = BOARD_NUM_SPACES - 1;

pub const BOARD_CELL_WIDTH_PX: f32 = 92.0;
pub const BOARD_CELL_WIDTH_PX_HALF: f32 = BOARD_CELL_WIDTH_PX / 2.0;
pub const BOARD_CELL_HEIGHT_PX: f32 = 70.0;
pub const BOARD_CELL_HEIGHT_PX_HALF: f32 = BOARD_CELL_HEIGHT_PX / 2.0;

// audio
pub const AUDIO_ERROR_ID: &str = "error";
pub const AUDIO_ERROR_URI: &str = "assets/audio/error.mp3";
pub const AUDIO_JUMP_URI: &str = "assets/audio/jump.mp3";

// UI images
pub const RESET_URI: &str = "/assets/images/return.png";

// death animation
pub const DEATH_DURATION: u32 = 200;
pub const DEATH_ALPHA: f32 = 0.0;
// pub const DEATH_SCALE: Vec2 = vec2(0.0, 0.0);

pub const BUZZ_STATE_DURATION: u32 = 20; // frames

pub const DROP_GRAVITY_PER_DT: f32 = -0.6;
pub const DROP_BOUNCE_DAMPENING: f32 = 0.3;
pub const DROP_HEIGHT_MIN: f32 = 300.0;
pub const DROP_HEIGHT_VARIANCE: f32 = 400.0;
pub const DROP_TERMINAL_VEL: f32 = 20.0;

pub const EXCITED_HOP_POWER: f32 = 5.0;
pub const EXCITED_HOP_INTERVAL: u32 = 30;

pub const JUMP_HEIGHT: f32 = 60.0;
pub const JUMP_DURATION: u32 = 20;

pub const RESET_SPIN_DURATION: u32 = 20;
