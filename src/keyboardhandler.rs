use std::collections::HashMap;

use sdl2::keyboard::Keycode;

pub struct KeyboarHandler {
    keystate: HashMap<Keycode, bool>,
}

impl KeyboarHandler {
    pub fn new() -> Self {
        let keystate = HashMap::new();
        Self { keystate }
    }
    pub fn toggle_pressed(&mut self, key: &Keycode) {
        *self.keystate.entry(key.to_owned()).or_insert(false) = true;
    }
    pub fn toggle_released(&mut self, key: &Keycode) {
        *self.keystate.entry(key.to_owned()).or_insert(true) = false;
    }
    pub fn is_pressed(&self, key: &Keycode) -> &bool {
        self.keystate.get(key).unwrap_or(&false)
    }
}
