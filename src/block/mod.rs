mod entity;
pub mod redstone;

pub use crate::block::entity::BlockEntity;

// pub trait Block {
//     fn get_position(&self) -> (i32, i32);
//     fn set_position(&mut self, position: (i32, i32));
//
//     fn get_rotation(&self) -> Facing;
//     fn set_rotation(&mut self, rotation: Facing);
//
//     fn get_solid(&self) -> Option<Box<dyn Solid>>;
//     fn get_block_entity(&self) -> Option<Box<dyn BlockEntity>>;
//
//     fn get_hitbox(&self) -> String;
//     fn get_name(&self) -> String;
//     fn set_name(&mut self, name: String);
//
//     fn is_sticky(&self) -> bool;
//     fn is_movable(&self) -> bool;
//
//     fn get_power_source(&self) -> Box<dyn Block>;
//     fn set_power_source(&mut self, powered_by: Box<dyn Block>);
//
//     fn get_redstone(&self) -> Option<Box<&RedstoneComponent>>;
// }
//
// pub trait Solid {
//     fn get_powered(&self) -> bool;
//     fn set_powered(&mut self, powered: bool) -> bool;
// }

#[allow(dead_code)]
pub struct Block<'a> {
    name: String,
    position: (i32, i32),
    rotation: Facing,

    is_solid: bool,
    powered: i8,
    power_source: Option<&'a Block<'a>>,
    redstone: Option<redstone::RedstoneComponent<'a>>,

    blockentity: Option<Box<dyn BlockEntity>>,

    hitbox: String,
    sticky: bool,
    movable: bool,
}

#[derive(Clone)]
pub enum Facing {
    North,
    South,
    East,
    West,
}
