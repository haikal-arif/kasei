use crate::decoratedtexture::{DecoratedTexture, TextureMetadata};

use sdl2::{libc::time_t, rect::Rect, render::Texture};

pub struct AnimatedTexture<'a> {
    spritesheet: &'a DecoratedTexture<'a>,
    currentframe: Rect,
    animation_metadata: AnimationMetadata,
    timer: time_t,
}
impl<'a> AnimatedTexture<'a> {
    pub fn new(
        spritesheet: &'a DecoratedTexture<'a>,
        animation_metadata: AnimationMetadata,
    ) -> Self {
        let (sprite_width, sprite_height) = spritesheet.get_sprite_size();
        let currentframe = Rect::new(0, 0, sprite_width, sprite_height);
        Self {
            spritesheet,
            currentframe,
            animation_metadata,
            timer: Default::default(),
        }
    }
    pub fn get_spritesheet(&self) -> &Texture<'a> {
        &self.spritesheet.get_texture()
    }
    pub fn get_spritesheet_metadata(&self) -> &TextureMetadata {
        &self.spritesheet.get_metadata()
    }

    pub fn get_sprite_size(&self) -> (u32, u32) {
        self.spritesheet.get_sprite_size()
    }
    pub fn update_frame(&mut self, delta_time: time_t) {
        self.timer += delta_time;

        // example:
        // We want 20 fps -> 20 frame / 1000 millisecond -> 50 millisecond/frame
        // Every 50 millis we change frame; timer / frameperiod => timer * fps/1000
        let total_frame_index = (self.timer as u32 * self.animation_metadata.fps) / 1000;
        let actual_frame_index = total_frame_index % self.animation_metadata.total_frame;
        let x_coord_in_spritesheet = actual_frame_index * self.get_sprite_size().0;
        self.currentframe.set_x(x_coord_in_spritesheet as i32);
    }
    pub fn get_frame(&self) -> Rect {
        self.currentframe
    }
}
#[derive(Debug, Clone)]
pub struct AnimationMetadata {
    pub fps: u32,
    pub total_frame: u32,
}
