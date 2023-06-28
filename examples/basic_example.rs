extern crate glium;

#[macro_use]
extern crate ascii_opengl_rust;

//------------------ My stuff --------------------------
mod basic_example_res;
use basic_example_res::game_init::game_init;
use basic_example_res::game_loop::game_loop;
// -----------------------------------------------------


fn main() {
    let mut pause = false;
    // let mut last_mouse_pos = (0, 0);

    // Main loop
    init_engine!(game_loop!(game_loop, &mut pause), game_init!(game_init), "examples/basic_example_res/assets");
}
