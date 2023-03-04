use super::assetsmanager::TextureID;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum MyID {
    NPC,
}

impl TextureID for MyID {}
