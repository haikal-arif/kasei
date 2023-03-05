use crate::decoratedtexture::{DecoratedTexture, TextureMetadata};

use sdl2::{libc::time_t, rect::Rect, render::Texture};

/// AnimatedTexture is an object responsible for handling texture animation

pub struct AnimatedTexture<'a> {
    spritesheet: &'a DecoratedTexture<'a>,
    current_frame: Rect,
    animation_metadata: AnimationMetadata,
    timer: time_t,
}
impl<'a> AnimatedTexture<'a> {
    pub fn new(
        spritesheet: &'a DecoratedTexture<'a>,
        animation_metadata: AnimationMetadata,
    ) -> Self {
        let (sprite_width, sprite_height) = spritesheet.sprite_size();
        let current_frame = Rect::new(0, 0, sprite_width, sprite_height);
        Self {
            spritesheet,
            current_frame,
            animation_metadata,
            timer: Default::default(),
        }
    }
    pub fn spritesheet(&self) -> &Texture<'a> {
        &self.spritesheet.texture()
    }
    pub fn spritesheet_metadata(&self) -> &TextureMetadata {
        &self.spritesheet.metadata()
    }

    pub fn sprite_size(&self) -> (u32, u32) {
        self.spritesheet.sprite_size()
    }

    pub fn update_frame(&mut self, delta_time: time_t) {
        self.timer += delta_time;

        // example:
        // We want 20 fps -> 20 frame / 1000 millisecond -> 50 millisecond/frame
        // Every 50 millis we change frame; timer / frameperiod => timer * fps/1000
        let total_frame_index = (self.timer as u32 * self.animation_metadata.fps) / 1000;
        let actual_frame_index = total_frame_index % self.animation_metadata.total_frame;
        let x_coord_in_spritesheet = actual_frame_index * self.sprite_size().0;
        self.current_frame.set_x(x_coord_in_spritesheet as i32);
    }

    /// Call this function for canvas.copy_ex src
    /// ```rust
    /// let _ = canvas.copy_ex(
    ///     texture: texture.spritesheet(),
    /// --> src: texture.current_frame(), <--
    ///     dst: position,
    ///     angle: angle,
    ///     center: center,
    ///     flip_horizontal,
    ///     flip_vertical,
    /// );
    /// ```
    pub fn current_frame(&self) -> Rect {
        self.current_frame
    }
}
#[derive(Debug, Clone)]
pub struct AnimationMetadata {
    pub fps: u32,
    pub total_frame: u32,
}
