use std::vec::IntoIter;

use crate::gameobject::{GameObject, GameObjectCreator};

pub struct ObjectPool<Creation: GameObject> {
    pool: Vec<Option<Creation>>,
    full: bool,
}
impl<Creation: GameObject> ObjectPool<Creation> {
    pub fn new() -> Self {
        let pool = Vec::with_capacity(50);
        let full = false;
        Self { pool, full }
    }

    pub fn spawn<Creator: GameObjectCreator<Creation = Creation>>(
        &mut self,
        object_creator: Creator,
    ) -> Result<(), String> {
        if self.full {
            return Err("Pool is full".to_string());
        }
        for (idx, room) in self.pool.iter().enumerate() {
            if room.is_none() {
                self.pool[idx] = Some(object_creator.create());
                return Ok(());
            }
        }
        self.full = true;
        Err("Pool is full".to_string())
    }
    pub fn get_pool_mut(&mut self) -> &mut Vec<Option<Creation>> {
        &mut self.pool
    }

    pub fn get_pool(&self) -> &Vec<Option<Creation>> {
        &self.pool
    }
}

impl<Creation: GameObject> IntoIterator for ObjectPool<Creation> {
    type Item = Option<Creation>;
    type IntoIter = IntoIter<Option<Creation>>;
    fn into_iter(self) -> Self::IntoIter {
        self.pool.into_iter()
    }
}
