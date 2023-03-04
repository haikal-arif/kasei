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
}
pub struct TextureMetadata {
    pub size: (u32, u32),
    pub frames_per_anim: u32,
}
