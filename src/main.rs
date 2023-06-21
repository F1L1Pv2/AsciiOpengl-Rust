#[macro_use]
extern crate glium;

//------------------ My stuff --------------------------
mod engine;
use engine::core::{ init, run_event_loop };
use engine::example::game_loop::game_loop;
use engine::example::game_init::game_init;
// -----------------------------------------------------

fn main() {
    let (
        terminal_res,
        terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        game,
    ) = init();

    let mut pause = false;

    // Main loop
    run_event_loop(
        terminal_res,
        terminal_fb,
        event_loop,
        display,
        program,
        ui_program,
        params,
        ui_params,
        game,
        move |device_state, terminal_res, game| {
            game_loop(device_state, terminal_res, game, &mut pause);
        },
        move |game, display| {
            game_init(game, display);
        }
    );
}
