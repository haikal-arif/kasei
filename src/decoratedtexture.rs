use sdl2::render::Texture;

pub struct DecoratedTexture<'a> {
    texture: Texture<'a>,
    metadata: TextureMetadata,
}
impl<'a> DecoratedTexture<'a> {
    pub fn new(texture: Texture<'a>, metadata: TextureMetadata) -> Self {
        Self { texture, metadata }
    }
    pub fn get_texture(&self) -> &Texture<'a> {
        &self.texture
    }
    pub fn get_metadata(&self) -> &TextureMetadata {
        &self.metadata
    }

    pub fn get_sprite_size(&self) -> (u32, u32) {
        self.metadata.sprite_size
    }
}
pub struct TextureMetadata {
    pub sprite_size: (u32, u32),
    pub atlas_size: (u32, u32),
}
