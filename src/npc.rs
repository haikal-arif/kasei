use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::{event::Event, libc::time_t, rect::Rect, render::Canvas};

use crate::assetsmanager::{AssetsManager, TextureID};
use crate::decoratedtexture::DecoratedTexture;
use crate::gameobject::GameObjectCreator;
use crate::vector2::Vector2;

use crate::animatedtexture::{AnimatedTexture, AnimationMetadata};
use crate::gameobject::GameObject;
use crate::keyboardhandler::KeyboarHandler;

pub struct NPC<'a> {
    texture: AnimatedTexture<'a>,
    position: sdl2::rect::Rect,
    horizontal_flipped: bool,
    vertical_flipped: bool,
    rendered: bool,
    simulated: bool,
    inactive: bool,
    velocity: Vector2,
    custom_update: fn(&mut Self, time_t),
    keyboard_handler: KeyboarHandler,
}

impl<'a> NPC<'a> {
    pub fn new(
        texture: AnimatedTexture<'a>,
        position: sdl2::rect::Rect,
        horizontal_flipped: bool,
        vertical_flipped: bool,
        rendered: bool,
        simulated: bool,
        inactive: bool,
        velocity: Vector2,
        custom_update: fn(&mut Self, time_t),
    ) -> Self {
        let keyboard_handler = KeyboarHandler::new();
        NPC {
            texture,
            position,
            horizontal_flipped,
            vertical_flipped,
            rendered,
            simulated,
            inactive,
            velocity,
            custom_update,
            keyboard_handler,
        }
    }

    pub fn set_position(&mut self, new_coords: (i32, i32)) {
        self.position.set_x(new_coords.0);
        self.position.set_y(new_coords.1);
    }

    pub fn set_position_x(&mut self, new_x_coord: i32) {
        self.position.set_x(new_x_coord);
    }

    pub fn set_position_y(&mut self, new_y_coord: i32) {
        self.position.set_x(new_y_coord);
    }

    pub fn set_hflip(&mut self, val: bool) -> &Self {
        self.horizontal_flipped = val;
        self
    }

    pub fn set_vflip(&mut self, val: bool) {
        self.vertical_flipped = val;
    }

    pub fn position(&self) -> Rect {
        self.position
    }

    pub fn set_rendered(&mut self, is_rendered: bool) {
        self.rendered = is_rendered;
    }
    pub fn keyboard_handler(&self) -> &KeyboarHandler {
        &self.keyboard_handler
    }
    pub fn velocity(&self) -> &Vector2 {
        &self.velocity
    }
    pub fn set_velocity(&mut self, new_velocity: Vector2) {
        self.velocity = new_velocity;
    }
    pub fn texture(&self) -> &AnimatedTexture<'a> {
        &self.texture
    }
    pub fn texture_mut(&mut self) -> &mut AnimatedTexture<'a> {
        &mut self.texture
    }
}

impl<'a> GameObject for NPC<'a> {
    fn is_rendered(&self) -> bool {
        self.rendered
    }
    fn is_simulated(&self) -> bool {
        self.simulated
    }
    fn is_inactive(&self) -> bool {
        self.inactive
    }

    fn init(&mut self) {}

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => self.keyboard_handler.toggle_pressed(key),
            Event::KeyUp {
                keycode: Some(key), ..
            } => self.keyboard_handler.toggle_released(key),
            _ => {}
        }
    }

    fn update(&mut self, delta_time: time_t) {
        (self.custom_update)(self, delta_time);
    }

    fn draw(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let canvas_size = canvas.output_size().unwrap();
        let dst = self.position;
        if dst.x() > (canvas_size.0 - self.texture.get_sprite_size().0) as i32 {
            self.rendered = false;
        }
        let _ = canvas.copy_ex(
            self.texture.get_spritesheet(),
            self.texture.get_frame(),
            self.position(),
            0.0,
            None,
            self.horizontal_flipped,
            self.vertical_flipped,
        );
    }
}

#[derive(Default)]
pub struct NPCCreator<'a> {
    texture: Option<AnimatedTexture<'a>>,
    position: (i32, i32),
    horizontal_flipped: bool,
    vertical_flipped: bool,
    rendered: bool,
    simulated: bool,
    velocity: Vector2,
    custom_update: Option<fn(&mut NPC, time_t)>,
}

impl<'a> NPCCreator<'a> {
    pub fn set_texture_from_manager<Identifier: TextureID>(
        mut self,
        texture_manager: &'a AssetsManager<'a, Identifier>,
        id: &Identifier,
        animation_metadata: AnimationMetadata,
    ) -> Result<Self, String> {
        let decorated_texture = texture_manager.get_texture(id).ok_or("Id not found")?;
        self.texture = Some(AnimatedTexture::new(decorated_texture, animation_metadata));
        Ok(self)
    }
    pub fn set_animated_texture_from_texture(
        mut self,
        texture: &'a DecoratedTexture<'a>,
        metadata: AnimationMetadata,
    ) -> Self {
        self.texture = Some(AnimatedTexture::new(texture, metadata));
        self
    }

    pub fn set_animated_texture_directly(mut self, texture: AnimatedTexture<'a>) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn flip_horizontal(mut self) -> Self {
        self.horizontal_flipped = true;
        self
    }

    pub fn flip_vertical(mut self) -> Self {
        self.vertical_flipped = true;
        self
    }

    pub fn rendered(mut self) -> Self {
        self.rendered = true;
        self
    }

    pub fn simulated(mut self) -> Self {
        self.simulated = true;
        self
    }
    pub fn set_position(mut self, coords: (i32, i32)) -> Self {
        self.position = coords;
        self
    }

    pub fn set_velocity(mut self, velocity: (f32, f32)) -> Self {
        self.velocity = Vector2::new(velocity.0, velocity.1);
        self
    }

    pub fn set_custom_update(mut self, custom_func: fn(&mut NPC, time_t)) -> Self {
        self.custom_update = Some(custom_func);
        self
    }
}

impl<'a> GameObjectCreator for NPCCreator<'a> {
    type Creation = NPC<'a>;

    fn create(self) -> Self::Creation {
        let texture = self.texture.expect("Texture should be initialized");

        let position = Rect::new(
            self.position.0,
            self.position.1,
            texture.get_sprite_size().0 * 4,
            texture.get_sprite_size().1 * 4,
        );
        let custom_update = match self.custom_update {
            Some(func) => func,
            None => |_: &mut NPC, _| {},
        };

        NPC::new(
            texture,
            position,
            self.horizontal_flipped,
            self.vertical_flipped,
            self.rendered,
            self.simulated,
            false,
            self.velocity,
            custom_update,
        )
    }
}
