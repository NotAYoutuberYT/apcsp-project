extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{
    contains_point_cache_bounds, MovingObject, RectObject, RigidBody, StaticObject, Vector2,
};

const WINDOW_WIDTH: usize = 0xff * 4;
const WINDOW_HEIGHT: usize = 0xff * 3;
const FRAME_LIMIT_MILLIS: u64 = 1000 / 144;

const PLAYER_WALKING_SPEED: f64 = 3.2;
const PLAYER_RUNNING_SPEED: f64 = 5.0;

const JUMP_BUFFER_HUNDRETHSECS: f64 = 0.0005;
const JUMP_FORCE: f64 = 5.0;

const GRAVITY_MOVING_UP: f64 = 1.0 / 7.8;
const GRAVITY_MOVING_DOWN: f64 = 1.0 / 4.5;
const VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT: f64 = -1.0 / 2.5;

const DIRECTIONAL_COLLISION_DEPTH: f64 = 7.5;

fn main() {
    // our player
    let mut player = RigidBody {
        center: Vector2::new(400.0, 300.0),
        width: 20.0,
        height: 40.0,

        velocity: Vector2::new(0.0, 0.0),
        density: 0.0,
        static_friction: false,
    };

    let static_objects = [
        StaticObject {
            center: Vector2::new(510.0, 75.0),
            width: 700.0,
            height: 150.0,
        },
        StaticObject {
            center: Vector2::new(430.0, 185.0),
            width: 80.0,
            height: 80.0,
        },
        StaticObject {
            center: Vector2::new(360.0, 650.0),
            width: 50.0,
            height: 225.0,
        },
        StaticObject {
            center: Vector2::new(410.0, 562.5),
            width: 50.0,
            height: 50.0,
        },
    ];

    let mut moving_objects = [
        MovingObject::new(
            Vector2::new(300.0, 245.0),
            Vector2::new(265.0, 575.0),
            80.0,
            35.0,
            220.0,
            false,
        ),
        MovingObject::new(
            Vector2::new(615.0, 400.0),
            Vector2::new(800.0, 545.0),
            100.0,
            35.0,
            220.0,
            false,
        ),
    ];

    // our window :)
    let mut window = Window::new(
        "Platformer - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|error| {
        panic!("Error opening window: {}", error);
    });

    // configure the window
    window.limit_update_rate(Some(std::time::Duration::from_millis(FRAME_LIMIT_MILLIS)));
    window.topmost(true);
    window.set_position(20, 20);

    // this will be where we write out pixel values
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // how long each frame takes (in hundreths of a seconds)
    let mut frame_time: f64 = 0.0;

    // jump buffers make movement feel a little better
    let mut jump_buffer: f64 = 0.0;

    // cache object bounds
    let static_object_bounds: Vec<(f64, f64, f64, f64)> = static_objects
        .iter()
        .map(|object| object.bounds())
        .collect();

    let mut player_bounds: (f64, f64, f64, f64);

    // this prevent "bouncing" on downward moving platforms
    let mut stuck_platform: Option<MovingObject> = None;

    //
    // game loop starts here
    //

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // used to measure the frame time
        let now = std::time::Instant::now();

        let gravity: f64;

        // find gravity
        gravity = match player.velocity.y <= 0.0 {
            false => GRAVITY_MOVING_UP,
            true => GRAVITY_MOVING_DOWN,
        };

        // move the player (we integrate the player's movement instead of approximating
        // to make the physics continuous and therefore frame-independent)
        let mut movement_vector = Vector2::new(0.0, 0.0);
        movement_vector.y =
            gravity * frame_time * frame_time / 2.0 + frame_time * player.velocity.y;
        player.move_by(&movement_vector);

        // update velocity
        player.velocity.y -= gravity * frame_time;

        // recache bounds for physics
        player_bounds = player.bounds();

        // find movement speed
        let current_speed: f64 = match window.is_key_down(Key::LeftShift) {
            false => PLAYER_WALKING_SPEED,
            true => PLAYER_RUNNING_SPEED,
        };

        // apply movement speed
        if window.is_key_down(Key::A) {
            player.center.x -= current_speed * frame_time;
        }
        if window.is_key_down(Key::D) {
            player.center.x += current_speed * frame_time;
        }

        // update moving platforms
        for moving_object in &mut moving_objects {
            (moving_object.update(frame_time));
        }

        // move into the platform we're stuck to if it exists
        if let Some(mut stuck_obj) = stuck_platform {
            if player.collides_with_x(&stuck_obj) {
                stuck_obj.update(frame_time);
                player.center.y =
                    stuck_obj.bounds().3 + player.height / 2.0 - DIRECTIONAL_COLLISION_DEPTH / 2.0;
            }
        }

        stuck_platform = None;

        //
        // collision handling here
        //

        let mut on_object = false;
        let mut under_object = false;

        for object in &moving_objects {
            let bounds = object.bounds();

            // if we collide with the object, decide the best
            // way to move ourselves outside of the object
            if player.collides_with(object) {
                if player_bounds.1 <= bounds.0 + DIRECTIONAL_COLLISION_DEPTH {
                    player.center.x = bounds.0 - player.width / 2.0;
                } else if player_bounds.0 >= bounds.1 - DIRECTIONAL_COLLISION_DEPTH {
                    player.center.x = bounds.1 + player.width / 2.0;
                }
                // if we're on top of a moving object, move with it
                else if player_bounds.2 >= bounds.3 - DIRECTIONAL_COLLISION_DEPTH {
                    player.center.x += object.prev_move.x;
                    player.center.y = bounds.3 + player.height / 2.0;

                    stuck_platform = Some(object.clone());

                    on_object = true;
                } else if player_bounds.3 <= bounds.2 + DIRECTIONAL_COLLISION_DEPTH {
                    player.center.y = bounds.2 - player.height / 2.0;

                    under_object = true;
                }
            }
        }

        for i in 0..static_objects.len() {
            let bounds = static_object_bounds[i];

            // if we collide with the object, decide the best
            // way to move ourselves outside of the object
            if player.collides_with(&static_objects[i]) {
                if player_bounds.0 >= bounds.1 - DIRECTIONAL_COLLISION_DEPTH {
                    player.center.x = bounds.1 + player.width / 2.0;
                } else if player_bounds.1 <= bounds.0 + DIRECTIONAL_COLLISION_DEPTH {
                    player.center.x = bounds.0 - player.width / 2.0;
                } else if player_bounds.2 >= bounds.3 - DIRECTIONAL_COLLISION_DEPTH {
                    player.center.y = bounds.3 + player.height / 2.0;

                    on_object = true;
                } else if player_bounds.3 <= bounds.2 + DIRECTIONAL_COLLISION_DEPTH {
                    player.center.y = bounds.2 - player.height / 2.0;

                    under_object = true;
                }
            }
        }

        jump_buffer -= frame_time;

        // if space is pressed, start jump buffer
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            jump_buffer = JUMP_BUFFER_HUNDRETHSECS;
        }

        if on_object && jump_buffer > 0.0 {
            player.velocity.y = JUMP_FORCE;
            jump_buffer = 0.0;
            stuck_platform = None; // if we jump, unstick ourselves
        } else if on_object {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        } else if under_object {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        }

        // recache bounds for graphics
        player_bounds = player.bounds();

        // this is for testing purposes
        if player.center.y < -20.0 {
            player.center = Vector2::new(400.0, 300.0);
        }

        //
        // graphics rendering below
        //

        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                let rgb: u32;
                let point = Vector2::new(x as f64, (WINDOW_HEIGHT - y) as f64);

                let mut static_object_collision: bool = false;
                let mut moving_object_collision: bool = false;

                // determine collision with static objects
                for bounds in &static_object_bounds {
                    if contains_point_cache_bounds(&point, &bounds) {
                        static_object_collision = true;
                    }
                }

                // determine collision with moving objects
                for moving_object in &moving_objects {
                    if moving_object.contains_point(&point) {
                        moving_object_collision = true;
                    }
                }

                if contains_point_cache_bounds(&point, &player_bounds) {
                    rgb = 0xff0000;
                } else if moving_object_collision {
                    rgb = 0xff00;
                } else if static_object_collision {
                    rgb = 0xff;
                } else {
                    rgb = 0x200020;
                }

                window_buffer[y * WINDOW_WIDTH + x] = rgb;
            }
        }

        // update our window with our pixel values
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|error| {
                panic!("Error updating window: {}", error);
            });

        // update how long the frame took
        frame_time = now.elapsed().as_micros() as f64 / 10000.0;
    }
}
