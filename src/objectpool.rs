use crate::gameobject::GameObject;

pub struct ObjectPool<Creation: GameObject> {
    pool: Vec<Option<Creation>>,
}
impl<Creation: GameObject> ObjectPool<Creation> {
    pub fn new() -> Self {
        let pool = Vec::new();
        Self { pool }
    }

    pub fn insert(&mut self, object: Creation) {
        for (idx, room) in self.pool.iter().enumerate() {
            if room.is_none() {
                self.pool[idx] = Some(object);
                return;
            }
        }
    }
}