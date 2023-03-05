use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

use crate::objectpool::ObjectPool;

use super::gameobject::GameObject;

// Struct for handling event
pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;
        Ok(Self { event_pump })
    }
    pub fn handle_events<Creation: GameObject>(
        &mut self,
        object_pool: &mut ObjectPool<Creation>,
    ) -> bool {
        let mut running = true;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
            EventHandler::handle_event(object_pool, &event);
        }
        running
    }

    fn handle_event<Creation: GameObject>(object_pool: &mut ObjectPool<Creation>, event: &Event) {
        let actual_pool = object_pool.pool_mut();
        for idx in 0..actual_pool.len() {
            if let Some(object) = &mut actual_pool[idx] {
                if object.is_inactive() {
                    actual_pool[idx] = None;
                    continue;
                }
                object.handle_event(event)
            }
        }
    }
}
