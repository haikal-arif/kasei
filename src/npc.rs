use sdl2::{event::Event, libc::time_t, render::Canvas, video::Window};

use crate::{AssetsManager, DecoratedTexture, GameObject, TextureID, Vector2};

pub struct NPC<'a> {
    texture: &'a DecoratedTexture<'a>,
    position: Vector2,
}

impl<'a> NPC<'a> {
    pub fn new<Key: TextureID>(
        assets_manager: &'a AssetsManager<Key>,
        texture_id: &Key,
        starting_position: Vector2,
    ) -> Self {
        let texture = assets_manager.get_texture(texture_id).unwrap();
        NPC {
            texture: texture,
            position: starting_position,
        }
    }
}

impl<'a> GameObject for NPC<'a> {
    fn init(&mut self) {}
    fn update(&mut self, delta_time: time_t, event: &Event) {}
    fn draw(&self, canvas: &mut Canvas<Window>) {}
}
