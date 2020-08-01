#![allow(dead_code)]
#![allow(unused)]

mod assets;
mod board;
mod constants;
mod draw;
mod sprites;
mod types;
mod update;
mod utils;

use crate::constants::*;
use crate::types::*;
use kit::*;
use rand;
use std::time::Duration;

const TITLE: &str = "Peg Game";

const DT_NANOS: u64 = 1_000_000_000 / 60;
const DT: Duration = Duration::from_nanos(DT_NANOS);

struct App {
  state: State,
}

impl KApp for App {
  fn new() -> Self {
    Self {
      state: Default::default(),
    }
  }

  fn init(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;

    assets::init(ctx, state);
    sprites::init(ctx, state);

    // set initial pegs on board
    utils::populate(state);
  }

  fn frame(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;
    update::update(ctx, state);
    // TODO pegs kinda blend together when overlapping; differentiate somehow
    draw::draw(ctx, state);
  }
}

fn main() {
  kit::run::<App>(KAppDesc {
    window_title: TITLE.to_string(),
    ..Default::default()
  });
}
