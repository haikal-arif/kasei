use sdl2::{event::Event, libc::time_t, EventPump, Sdl};

use crate::objectpool::ObjectPool;

use super::gameobject::GameObject;

pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;
        Ok(Self { event_pump })
    }
    pub fn handle_events<Creation: GameObject>(&mut self, object_pool: &mut ObjectPool<Creation>) {
        for event in self.event_pump.poll_iter() {
            EventHandler::handle_event(object_pool, &event);
        }
    }

    fn handle_event<Creation: GameObject>(object_pool: &mut ObjectPool<Creation>, event: &Event) {
        let actual_pool = object_pool.get_pool_mut();
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
