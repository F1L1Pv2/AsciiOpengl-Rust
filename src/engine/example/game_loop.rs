// WIP
use device_query::{ DeviceState, DeviceQuery, Keycode };
use super::super::core::Game;

pub fn game_loop(
    device_state: &DeviceState,
    terminal_res: (u32, u32),
    game: &mut Game,
    pause: &mut bool,
    // last_mouse_pos: &mut (i32, i32),
) {

    // let mouse_sensitive = 0.1;

    let mut move_vector = [0, 0, 0];
    let mut mouse_vector = [0, 0];

    let keys: Vec<Keycode> = device_state.get_keys();

    //get mouse delta
    // let mouse_state = device_state.get_mouse();

    // let mouse_pos = mouse_state.coords;

    // println!("mouse pos: {:?}", mouse_pos);

    // let mouse_delta = (mouse_pos.0 - last_mouse_pos.0, mouse_pos.1 - last_mouse_pos.1);


    // //normalize vector without termianl_res
    // let mouse_delta_len = (mouse_delta.0 as f32 * mouse_delta.0 as f32 + mouse_delta.1 as f32 * mouse_delta.1 as f32).sqrt();

    // let new_mouse_delta: (f32, f32);

    // if mouse_delta_len != 0.0 {
    //     new_mouse_delta = (mouse_delta.0 as f32 / mouse_delta_len, mouse_delta.1 as f32 / mouse_delta_len) as (f32, f32);
    // }else{
    //     new_mouse_delta = (0.0, 0.0);
    // }

    // let mouse_delta = new_mouse_delta;

    // println!("mouse delta: {:?}", mouse_delta);

    // *last_mouse_pos = mouse_pos;

    for key in keys {
        match key {
            Keycode::W => {
                move_vector[2] = 1;
            }
            Keycode::S => {
                move_vector[2] = -1;
            }
            Keycode::A => {
                move_vector[0] = -1;
            }
            Keycode::D => {
                move_vector[0] = 1;
            }
            Keycode::Space => {
                move_vector[1] = 1;
            }
            Keycode::LShift => {
                move_vector[1] = -1;
            }
            Keycode::I => {
                mouse_vector[1] = 1;
            }
            Keycode::K => {
                mouse_vector[1] = -1;
            }
            Keycode::J => {
                mouse_vector[0] = -1;
            }
            Keycode::L => {
                mouse_vector[0] = 1;
            }
            Keycode::Escape => {
                //clear the terminal
                // print!("\x1B[2J\x1B[1;1H");
                // std::process::exit(0);
                *pause = !*pause;
            }
            Keycode::Q => {
                game.set_scene(0);
            }
            Keycode::E => {
                game.set_scene(1);
            }
            _ => (),
        }
    }
    if !*pause {
        game.camera.update(terminal_res, move_vector, mouse_vector);
        // game.camera.player_rot[0] += mouse_delta.1 as f32 * mouse_sensitive;
        // game.camera.player_rot[1] += mouse_delta.0 as f32 * mouse_sensitive;
    }
}
