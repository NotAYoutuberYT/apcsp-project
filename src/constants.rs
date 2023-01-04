//
// config constants
//

// colors :)
pub const NORMAL_PLAYER_COLOR: u32 = 0xf00000;
pub const SPRINTING_PLAYER_COLOR: u32 = 0xff3c60;
pub const MOVING_OBJECT_COLOR: u32 = 0xff00;
pub const STATIC_OBJECT_COLOR: u32 = 0xff;
pub const BACKGROUND_COLOR: u32 = 0x200020;

// window stuff
pub const WINDOW_WIDTH: usize = 260 * 4;
pub const WINDOW_HEIGHT: usize = 260 * 3;
pub const FPS: f64 = 144.0;

// player stuff
pub const PLAYER_WALKING_ACCEL: f64 = 2.4;
pub const PLAYER_RUNNING_ACCEL: f64 = 3.6;
pub const PLAYER_AIR_ACCELL_RATIO: f64 = 0.1;

// physics stuff
pub const FRICTION_GROUND: f64 = 0.7;
pub const FRICTION_AIR: f64 = 0.08;

// stuff pertaining to keeping the camera focused on the player
pub const PERCENT_SCREEN_PLAYER_IN_X: f64 = 18.0;
pub const PERCENT_SCREEN_PLAYER_IN_Y: f64 = 17.5;
pub const PLAYER_FOCUS_X_OFFSET: f64 = 0.0;
pub const PLAYER_FOCUS_Y_OFFSET: f64 = -230.0;
pub const CAMERA_MOVING_EASING_X: f64 = 1.0 / 750.0;
pub const CAMERA_MOVING_EASING_Y: f64 = 1.0 / 1300.0;

// jump stuff
pub const JUMP_FORCE: f64 = 5.0;
pub const JUMP_BUFFER_HUNDRETHSECS: f64 = 0.0006;

// gravity
pub const GRAVITY_MOVING_UP: f64 = -1.0 / 7.8;
pub const GRAVITY_MOVING_DOWN: f64 = -1.0 / 4.5;
pub const VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT: f64 = -1.0 / 2.5;

// increasing this may increase performance on low fps
// but will make player snap to the edges of platforms
pub const COLLISION_DEPTH_BASE: f64 = 3.5;
pub const COLLISION_MAX_LOOPS: u32 = 12;

//
// don't touch these constants
//

pub const MIN_X_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
pub const MAX_X_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
pub const MIN_Y_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;
pub const MAX_Y_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;

pub const FRAME_LIMIT_MILLIS: u64 = (1000.0 / FPS) as u64;