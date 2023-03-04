use std::collections::HashMap;

use kasei::prelude::*;

mod textureidentifier;
mod npc;

use npc::NPC;
use textureidentifier::MyID;
fn main() -> Result<(), String> {
    let context_manager = ContextManager::new()?;
    let window_manager = WindowManager::new(
        &context_manager.video_subsystem,
        "Amz plz download",
        800,
        600,
        false,
    )?;
    let texture_creator = window_manager.get_canvas().texture_creator();
    let my_map: HashMap<MyID, DecoratedTexture> = HashMap::new(); 
    let mut assets_manager = AssetsManager::new(&texture_creator, my_map);
    let object_pool: ObjectPool<NPC> = ObjectPool::new();
    let metadata = TextureMetadata {
        size: (32, 32),
        frames_per_anim: 4,
    };
    let texture = assets_manager.load_texture(&MyID::NPC, "hombre", metadata)?;
    let event_pump = context_manager.sdl_context.event_pump()?;
    let running = true;
    while running {}
    Ok(())
}
