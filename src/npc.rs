use sdl2::rect::Point;
use sdl2::{event::Event, libc::time_t, rect::Rect, render::Canvas};

use crate::assetsmanager::{AssetsManager, TextureID};
use crate::decoratedtexture::DecoratedTexture;
use crate::gameobject::GameObjectCreator;
use crate::vector2::Vector2;

use crate::animatedtexture::{AnimatedTexture, AnimationMetadata};
use crate::gameobject::GameObject;

pub struct NPC<'a> {
    texture: AnimatedTexture<'a>,
    position_in_world: sdl2::rect::Rect,
    horizontal_flipped: bool,
    vertical_flipped: bool,
    rendered: bool,
    simulated: bool,
    inactive: bool,
    velocity: Vector2,
}

impl<'a> NPC<'a> {
    pub fn new(
        texture: AnimatedTexture<'a>,
        position_in_world: sdl2::rect::Rect,
        horizontal_flipped: bool,
        vertical_flipped: bool,
        rendered: bool,
        simulated: bool,
        inactive: bool,
        velocity: Vector2,
    ) -> Self {
        NPC {
            texture,
            position_in_world,
            horizontal_flipped,
            vertical_flipped,
            rendered,
            simulated,
            inactive,
            velocity,
        }
    }

    pub fn set_position(&mut self, new_coords: (i32, i32)) {
        self.position_in_world.set_x(new_coords.0);
        self.position_in_world.set_y(new_coords.1);
    }

    pub fn set_position_x(&mut self, new_x_coord: i32) {
        self.position_in_world.set_x(new_x_coord);
    }

    pub fn set_position_y(&mut self, new_y_coord: i32) {
        self.position_in_world.set_x(new_y_coord);
    }

    pub fn set_hflip(&mut self, val: bool) -> &Self {
        self.horizontal_flipped = val;
        self
    }

    pub fn set_vflip(&mut self, val: bool) {
        self.vertical_flipped = val;
    }

    pub fn get_position(&self) -> Rect {
        self.position_in_world
    }

    pub fn set_rendered(&mut self, is_rendered: bool) {
        self.rendered = is_rendered;
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

    fn update(&mut self, delta_time: time_t, event: &Event) {
        self.texture.update_frame(delta_time);
        let prev_x = self.position_in_world.x();
        let prev_y = self.position_in_world.y();

        let displacement_x = (delta_time as f32 * self.velocity.x() + 0.5) as i32;
        let displacement_y = (delta_time as f32 * self.velocity.y() + 0.5) as i32;

        let mut new_x = displacement_x + prev_x;
        let new_y = displacement_y + prev_y;

        if new_x > 768 {
            new_x = -128
        }
        if new_x < -128 {
            new_x = 672
        }

        self.position_in_world.set_x(new_x);
        self.position_in_world.set_y(new_y);

        println!("Previous {:?}", prev_x);
        println!("Displacement {:?}", displacement_x);
        println!("Position {:?}", new_x);
    }

    fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let _ = canvas.copy_ex(
            self.texture.get_spritesheet(),
            self.texture.get_frame(),
            self.get_position(),
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
    position_in_world: (i32, i32),
    horizontal_flipped: bool,
    vertical_flipped: bool,
    rendered: bool,
    simulated: bool,
    velocity: Vector2,
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
    pub fn set_position_in_world(mut self, coords: (i32, i32)) -> Self {
        self.position_in_world = coords;
        self
    }

    pub fn set_velocity(mut self, velocity: (f32, f32)) -> Self {
        self.velocity = Vector2::new(velocity.0, velocity.1);
        self
    }
}

impl<'a> GameObjectCreator for NPCCreator<'a> {
    type Creation = NPC<'a>;
    fn create(self) -> Self::Creation {
        let texture = self.texture.expect("Texture should be initialized");

        let mut position_in_world = Rect::new(
            0,
            0,
            texture.get_sprite_size().0 * 4,
            texture.get_sprite_size().1 * 4,
        );
        position_in_world.center_on(Point::new(
            self.position_in_world.0,
            self.position_in_world.1,
        ));
        NPC::new(
            texture,
            position_in_world,
            self.horizontal_flipped,
            self.vertical_flipped,
            self.rendered,
            self.simulated,
            false,
            self.velocity,
        )
    }
}
