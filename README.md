# kit

Kit is a very raw 3D game engine created for learning game engine coding and maybe someday it'll be a reasonable foundation for game jam projects.

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