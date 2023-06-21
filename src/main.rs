#[macro_use]
extern crate glium;

//------------------ My stuff --------------------------
mod engine;
use engine::core::{init, run_event_loop};
use engine::example::game_init::game_init;
use engine::example::game_loop::game_loop;
// -----------------------------------------------------

fn main() {

    let mut pause = false;
    // let mut last_mouse_pos = (0, 0);

    // Main loop
    run_event_loop(
        init(),
        move |device_state, terminal_res, game| {
            game_loop(device_state, terminal_res, game, &mut pause);
        },
        move |game, display| {
            game_init(game, display);
        },
    );
}
