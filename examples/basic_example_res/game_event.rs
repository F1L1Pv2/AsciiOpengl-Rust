//game_event.rs

use device_query::Keycode;

pub enum GameEvent{
    KeyUp(KeyUpEvent),
    KeyDown(KeyDownEvent),
}

pub struct KeyUpEvent{
    pub key: Keycode,
}

pub struct KeyDownEvent{
    pub key: Keycode,
}