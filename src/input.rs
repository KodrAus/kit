use glam::Vec2;

/// describes the most recent mouse button state
#[derive(Default)]
pub struct ButtonState {
    /// the number of presses during the previous frame
    pub prev_down: u32,
    /// the number of releases during the previous frame
    pub prev_up: u32,
    /// the number of presses during the current frame
    pub down: u32,
    /// the number of releases during the current frame
    pub up: u32,
}

impl ButtonState {
    pub(crate) fn frame_end(&mut self) {
        self.prev_down = self.down;
        self.prev_up = self.up;
        self.down = 0;
        self.up = 0;
    }
}

/// read from this struct to access information about mouse input state
#[derive(Default)]
pub struct MouseCtx {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,

    // TODO should there be a way to get mouse position in world coordinates? ie. reverse view & projeection?
    pub pos: Vec2,
    pub prev_pos: Vec2,

    /// contains the amount of mouse wheel movement since the previous frame
    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl MouseCtx {
    pub(crate) fn frame_end(&mut self) {
        if (self.left.down > 0) {
            println!("engine mouse down {}", self.left.down);
        }
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.prev_pos = self.pos;
        self.left.frame_end();
        self.middle.frame_end();
        self.right.frame_end();
    }
}

/// Holds input state. Read from this during a game update to consume player inputs.
#[derive(Default)]
pub struct InputCtx {
    pub mouse: MouseCtx,

    // TODO add multiple controllers
    pub l_stick: Vec2,
    pub r_stick: Vec2,

    // TODO should this be pub? Maybe hide it as an implementation detail
    pub quit: bool,
    // TODO replace these with actual keyboard state - what to do with the keys should be
    // a detail the game provides.
    pub dir_u: bool,
    pub dir_d: bool,
    pub dir_l: bool,
    pub dir_r: bool,
    pub action_pressed: bool,
    pub action_released: bool,
    // TODO touch input
}

/// describes a type of input the player may be using
pub enum InputType {
    MouseKeyboard,
    Gamepad, // TODO which kind? may be relevant for icons
    Touch,
}

pub fn preferred() -> InputType {
    // TODO https://www.gamasutra.com/blogs/ZachBurke/20151030/257920/The_5_Golden_Rules_of_Input.php
    InputType::MouseKeyboard
    // TODO detect gamepad
    // TODO detect touch
}
