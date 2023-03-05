use std::{collections::HashMap, hash::Hash};

use sdl2::{image::LoadTexture, render::TextureCreator, video::WindowContext};

use crate::decoratedtexture::{DecoratedTexture, TextureMetadata};

pub trait TextureID: Clone + Eq + Hash + PartialEq {}

pub struct AssetsManager<'a, Key: TextureID> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<Key, DecoratedTexture<'a>>,
}
impl<'a, Key: TextureID> AssetsManager<'a, Key> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        texture_map: HashMap<Key, DecoratedTexture<'a>>,
    ) -> Self {
        let textures = texture_map;
        Self {
            texture_creator,
            textures,
        }
    }
    pub fn load_texture(
        &mut self,
        texture_id: &Key,
        file_path: &str,
        metadata: TextureMetadata,
    ) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(file_path)?;
        let decorated_texture = DecoratedTexture::new(texture, metadata);
        self.textures
            .insert(texture_id.to_owned(), decorated_texture);
        Ok(())
    }
    pub fn get_texture(&self, texture_id: &Key) -> Option<&DecoratedTexture> {
        self.textures.get(texture_id)
    }
}
