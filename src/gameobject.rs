use sdl2::{libc::time_t, render::Canvas, video::Window, event::Event};


pub trait GameObject {
    fn init(&mut self);
    fn update(&mut self, delta_time: time_t, event: &Event);
    fn draw(&self, canvas: &mut Canvas<Window>);
}
pub trait GameObjectCreator {
    type Creation: GameObject;
    fn create(self) -> Self::Creation;
}