mod entity;
mod redstone;

pub use crate::block::entity::BlockEntity;

pub trait Block {
    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, position: (i32, i32));

    fn get_rotation(&self) -> Facing;
    fn set_rotation(&mut self, rotation: Facing);

    fn get_solid(&self) -> Option<Box<dyn Solid>>;
    fn get_block_entity(&self) -> Option<Box<dyn BlockEntity>>;

    fn get_hitbox(&self) -> String;
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);

    fn is_sticky(&self) -> bool;
    fn is_movable(&self) -> bool;
}

pub trait Solid {
    fn get_powered(&self) -> bool;
    fn set_powered(&mut self, powered: bool) -> bool;

    fn get_powered_by(&self) -> Box<dyn Block>;
    fn set_powered_by(&mut self, powered_by: Box<dyn Block>);
}

#[derive(Clone)]
pub enum Facing {
    North,
    South,
    East,
    West,
}
