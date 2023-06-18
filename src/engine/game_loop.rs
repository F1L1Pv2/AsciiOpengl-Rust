// WIP
use device_query::{ DeviceState, DeviceQuery, Keycode };
use super::camera::Camera;
use super::scene::Scene;

pub fn game_loop(
    device_state: &DeviceState,
    terminal_res: (u32, u32),
    camera: &mut Camera,
    _scene: &mut Scene,
) {

    let mut move_vector = [0, 0, 0];
    let mut mouse_vector = [0, 0];

    let keys: Vec<Keycode> = device_state.get_keys();

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
    camera.update(terminal_res, move_vector, mouse_vector);
}
