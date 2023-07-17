extern crate glium;

#[macro_use]
extern crate ascii_opengl_rust;

//------------------ My stuff --------------------------
mod basic_example_res;
use basic_example_res::game_init::game_init;
use basic_example_res::game_loop::game_loop;
use basic_example_res::game_event::GameEvent;
// -----------------------------------------------------

fn main() {
    let mut pause = false;

    let mut last_keys: Vec<device_query::Keycode> = Vec::new();

    let mut game_events: Vec<GameEvent> = Vec::new();

    // Main loop
    init_engine!(
        game_loop!(game_loop, &mut pause, &mut game_events, &mut last_keys),
        game_init!(game_init),
        "examples/basic_example_res/assets"
    );
}
