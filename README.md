# kit

Kit is a very raw 3D game engine. I made it to learn game engine coding, and someday I'd like it to be a reasonable foundation for game jam projects.

Kit will support 2D as well as 3D, but if you're looking for a 2D focused engine, I recommend checking out [ggez]() instead, as that seems really solid
and is way further along in development.

## TODO

Missing features abound. Use at your own risk (I wouldn't even describe this as usable at this point). Things that I'm currently planning to add:

- gamepad support
- full keyboard input support
- audio
- documentation might be nice /s
- sensible (or extensible? configurable?) limits for draw commands (they're highly arbitrary right now)
- complete vector math library (maybe I'll switch to glam later? my hand-rolled version has many holes and inconsistencies at the moment)
- investigate using wgpu-rs instead of sokol (wgpu-rs is likely a more Rust-centric approach)
- 3D model loading
- lighting (maybe)
- line, circle, point primitives in drawing commands are always drawn *over* models and *under* sprites - should fix depth handling
- hot-reloading of game for rapid iteration
- example projects
- multiple rendering layers (to overlay a gui, for example)
- asset un-loading (for area transitions and such)

## Getting started

```
use kit::*;

const TITLE: &str = "My Amazing Game";

struct App {
    // game state can go here
}

impl KApp for App {
    fn new() -> Self {
        Self {}
    }
    fn init(&mut self, ctx: &mut Ctx) {
        // initialization logic goes here
    }
    fn frame(&mut self, ctx: &mut Ctx) {
        let state = &mut self.state;
        // update logic goes here
    }
}

fn main() {
    run::<App>(KAppDesc {
        window_title: TITLE.to_string(),
        ..Default::default()
    });
}
```