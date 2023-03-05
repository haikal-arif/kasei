use sdl2::libc::time_t;

use crate::{gameobject::GameObject, objectpool::ObjectPool};

pub struct SceneEngineer {}

impl SceneEngineer {
    pub fn run_simulation<Creation: GameObject>(
        &self,
        object_pool: &mut ObjectPool<Creation>,
        delta_time: time_t,
    ) {
        for maybe_object in object_pool.pool_mut() {
            if let Some(object) = maybe_object {
                if object.is_simulated() {
                    object.update(delta_time)
                }
            }
        }
    }
}
