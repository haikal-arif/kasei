mod assetsmanager;
mod contextmanager;
mod decoratedtexture;
mod gameobject;
mod objectpool;
mod vector2;
mod windowmanager;

pub mod prelude {
    pub use crate::contextmanager::ContextManager;
    pub use crate::decoratedtexture::{DecoratedTexture, TextureMetadata};
    pub use crate::gameobject::{GameObject, GameObjectCreator};
    pub use crate::vector2::Vector2;
    pub use crate::windowmanager::WindowManager;
    pub use crate::assetsmanager::{AssetsManager, TextureID};
    pub use crate::objectpool::ObjectPool;
}

// trait Engineer<Creation: GameObject> {
//     fn simulate(
//         object_pool: &mut ObjectPool<Creation>,
//         event_pool: &mut EventPump,
//         delta_time: i32,
//         total_elapsed: i32,
//     );
// }
// trait Artist<Creation: GameObject> {
//     fn draw(&self, object_pool: &mut ObjectPool<Creation>, canvas: Canvas<Window>);
// }

// struct Producer;

// impl Producer {
//     fn spawn<Creator: GameObjectCreator<Creation = Creation>, Creation: GameObject>(
//         object_pool: &mut ObjectPool<Creation>,
//         creator: Box<Creator>,
//     ) {
//         object_pool.insert(creator.create());
//     }
// }
