use kasei::prelude::TextureID;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum MyID {
    NPC,
}

impl TextureID for MyID {}
