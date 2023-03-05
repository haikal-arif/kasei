use sdl2::{self, render::Canvas, video::Window};

use crate::{gameobject::GameObject, objectpool::ObjectPool};
pub struct Artist {}
impl Artist {
    pub fn draw<Creation: GameObject>(
        &self,
        object_pool: &mut ObjectPool<Creation>,
        canvas: &mut Canvas<Window>,
    ) {
        canvas.clear();
        let actual_pool = object_pool.get_pool_mut();
        for maybe_object in actual_pool {
            if let Some(object) = maybe_object {
                if object.is_rendered() {
                    object.draw(canvas)
                }
            }
        }
        canvas.present();
    }
}
