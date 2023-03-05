use std::collections::HashMap;

mod animatedtexture;
mod artist;
mod assetsmanager;
mod contextmanager;
mod decoratedtexture;
mod engineer;
mod eventhandler;
mod gameobject;
mod keyboardhandler;
mod npc;
mod objectpool;
mod textureidentifier;
mod vector2;
mod windowmanager;

use animatedtexture::AnimationMetadata;
use artist::Artist;
use assetsmanager::AssetsManager;
use contextmanager::ContextManager;
use decoratedtexture::{DecoratedTexture, TextureMetadata};
use engineer::SceneEngineer;
use eventhandler::EventHandler;
use npc::{NPCCreator, NPC};
use objectpool::ObjectPool;
use sdl2::{keyboard::Keycode, libc::time_t, pixels::Color};
use textureidentifier::MyID;
use windowmanager::WindowManager;

use crate::{gameobject::GameObject, vector2::Vector2};

fn main() -> Result<(), String> {
    let context_manager = ContextManager::new()?;
    let mut prev_millis = context_manager.timer_subsystem.ticks();
    let mut window_manager = WindowManager::new(
        &context_manager.video_subsystem,
        "Amz plz download",
        800,
        600,
        false,
    )?;
    let texture_creator = window_manager.get_canvas().texture_creator();
    let my_map: HashMap<MyID, DecoratedTexture> = HashMap::new();

    let mut assets_manager = AssetsManager::new(&texture_creator, my_map);
    let mut object_pool: ObjectPool<NPC> = ObjectPool::new();
    let metadata = TextureMetadata {
        sprite_size: (32, 32),
        atlas_size: (128, 96),
    };
    let animation_metadata = AnimationMetadata {
        fps: 10,
        total_frame: 4,
    };
    assets_manager.load_texture(&MyID::NPC(0), "assets/King.png", metadata.clone())?;
    assets_manager.load_texture(&MyID::NPC(1), "assets/Soldier.png", metadata)?;

    let texture = assets_manager.get_texture(&MyID::NPC(0)).unwrap();
    let another_texture = assets_manager.get_texture(&MyID::NPC(1)).unwrap();
    let npc_builder = NPCCreator::default()
        .set_animated_texture_from_texture(texture, animation_metadata.clone())
        .set_velocity((0.2, 0.0))
        .set_position((0, 360))
        .rendered()
        .simulated();

    let soldier = NPCCreator::default()
        .set_animated_texture_from_texture(another_texture, animation_metadata)
        .set_velocity((0.1, 0.0))
        .set_position((0, 200))
        .flip_horizontal()
        .rendered()
        .set_custom_update(my_custom_update)
        .simulated();
    let _ = object_pool.spawn(npc_builder);
    let _ = object_pool.spawn(soldier);
    let mut event_handler = EventHandler::new(&context_manager.sdl_context)?;
    let mut running = true;
    let mut curr_millis;
    let mut delta_time;
    window_manager
        .get_canvas_mut()
        .set_draw_color(Color::RGBA(0, 0, 0, 0));
    window_manager.get_canvas_mut().clear();
    window_manager.get_canvas_mut().present();
    let artist = Artist {};
    let scene_engineer = SceneEngineer {};
    while running {
        curr_millis = context_manager.timer_subsystem.ticks();
        delta_time = curr_millis - prev_millis;
        if delta_time < 16 {
            continue;
        }

        running = event_handler.handle_events(&mut object_pool);
        scene_engineer.run_simulation(&mut object_pool, delta_time as time_t);
        artist.draw(&mut object_pool, window_manager.get_canvas_mut());
        prev_millis = curr_millis;
    }
    Ok(())
}

fn my_custom_update(npc: &mut NPC, delta_time: time_t) {
    static BASE_VELOCITY: f32 = 0.2;
    match (
        npc.keyboard_handler().is_pressed(&Keycode::Up),
        npc.keyboard_handler().is_pressed(&Keycode::Down),
    ) {
        (&true, &false) => npc.set_velocity(Vector2::new(npc.velocity().x(), -BASE_VELOCITY)),
        (&false, &true) => npc.set_velocity(Vector2::new(npc.velocity().x(), BASE_VELOCITY)),
        (&true, &true) => npc.set_velocity(Vector2::new(npc.velocity().x(), -npc.velocity().y())),
        (&false, &false) => npc.set_velocity(Vector2::new(npc.velocity().x(), 0.0)),
    }

    match (
        npc.keyboard_handler().is_pressed(&Keycode::Left),
        npc.keyboard_handler().is_pressed(&Keycode::Right),
    ) {
        (&true, &false) => npc.set_velocity(Vector2::new(-BASE_VELOCITY, npc.velocity().y())),
        (&false, &true) => npc.set_velocity(Vector2::new(BASE_VELOCITY, npc.velocity().y())),
        (&true, &true) => npc.set_velocity(Vector2::new(-npc.velocity().x(), npc.velocity().y())),
        (&false, &false) => npc.set_velocity(Vector2::new(0.0, npc.velocity().y())),
    }

    npc.texture_mut().update_frame(delta_time);
    let prev_x = npc.position().x();
    let prev_y = npc.position().y();

    let displacement_x = (delta_time as f32 * npc.velocity().x()) as i32;
    let displacement_y = (delta_time as f32 * npc.velocity().y()) as i32;
    npc.set_velocity(Vector2::new(npc.velocity().x(), 0.0));
    let mut new_x = displacement_x + prev_x;
    let new_y = displacement_y + prev_y;

    if !npc.is_rendered() {
        new_x = -128;
        npc.set_rendered(true);
    }

    npc.set_position((new_x, new_y));
}
