use super::assetsmanager::TextureID;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum MyID {
    NPC(i32),
}

impl TextureID for MyID {}
