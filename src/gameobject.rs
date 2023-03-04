use sdl2::{event::Event, libc::time_t, render::Canvas, video::Window};

pub trait GameObject {
    fn init(&mut self);
    fn update(&mut self, delta_time: time_t, event: &Event);
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn is_rendered(&self) -> bool;
    fn is_simulated(&self) -> bool;
    fn is_inactive(&self) -> bool;
}
pub trait GameObjectCreator {
    type Creation: GameObject;
    fn create(self) -> Self::Creation;
}
