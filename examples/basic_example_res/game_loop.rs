// WIP
use ascii_opengl_rust::engine::core::Game;
use device_query::{ DeviceQuery, DeviceState, Keycode };
use crate::basic_example_res::game_event::{ GameEvent, KeyDownEvent, KeyUpEvent };

pub fn game_loop(
    device_state: &DeviceState,
    terminal_res: (u32, u32),
    game: &mut Game,
    display: &glium::Display,
    pause: &mut bool,
    game_events: &mut Vec<GameEvent>,
    last_keys: &mut Vec<Keycode>
) {
    // let mouse_sensitive = 0.1;

    let mut move_vector = [0, 0, 0];
    let mut mouse_vector = [0, 0];

    let keys: Vec<Keycode> = device_state.get_keys();

    // keys handler -----------------------------------------------------------------------

    keys.iter().for_each(|key| {
        if !last_keys.contains(key) {
            game_events.push(GameEvent::KeyDown(KeyDownEvent { key: key.clone() }));
        }
    });

    last_keys.iter().for_each(|key| {
        if !keys.contains(key) {
            game_events.push(GameEvent::KeyUp(KeyUpEvent { key: key.clone() }));
        }
    });

    *last_keys = keys.clone();

    // mouse handler ----------------------------------------------------------------------

    let mouse_pos = device_state.get_mouse().coords;

    let mut mouse_delta = (mouse_pos.0 as f32 / terminal_res.0 as f32 - 0.5, mouse_pos.1 as f32 / terminal_res.1 as f32 - 0.5);

    //normalize mouse delta
    
    // let mouse_delta_mag = (mouse_delta.0 * mouse_delta.0 + mouse_delta.1 * mouse_delta.1).sqrt();
    // let mut mouse_delta = (mouse_delta.0 / mouse_delta_mag, mouse_delta.1 / mouse_delta_mag);


    // if mouse_delta.1.abs() > 0.0{
    //     mouse_delta.0 /= mouse_delta.1.abs();
    // }
    if mouse_delta.0.abs() < 0.01 {
        mouse_delta.0 = 0.0;
    }

    // mouse_delta.0 =0

    if mouse_delta.1.abs() < 0.01 {
        mouse_delta.1 = 0.0;
    }

    // println!("{:?}", mouse_delta);
    // game events ------------------------------------------------------------------------

    game_events.iter().for_each(|event| {
        match event {
            GameEvent::KeyUp(event) => {
                match event.key {
                    Keycode::Escape => {
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
            _ => (),
        }
    });

    // game logic -------------------------------------------------------------------------

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
            _ => (),
        }
    }
    if !*pause {

        let aspect_ratio = terminal_res.0 as f32 / terminal_res.1 as f32;

        game.camera.update_by_speed(terminal_res, move_vector, mouse_vector);

        //rotate the camera
        game.camera.player_rot[0] += (mouse_delta.1 as f32) * 0.4;
        game.camera.player_rot[1] += (mouse_delta.0 as f32) * 0.4 * aspect_ratio;

        game.camera.update_self(terminal_res);

        display
            .gl_window()
            .window()
            .set_cursor_position(
                glium::glutin::dpi::PhysicalPosition::new(
                    (terminal_res.0 as f64) / 2.0,
                    (terminal_res.1 as f64) / 2.0
                )
            )
            .unwrap();
    }

    //clean up ---------------------------------------------------------------------------

    game_events.clear();
}
