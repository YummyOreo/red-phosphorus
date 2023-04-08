pub mod entity;
pub mod redstone;

use self::redstone::Component;
use crate::block::entity::BlockEntity;

pub trait Block<'a> {
    fn get_name(&self) -> &'a str;

    fn get_position(&self) -> &'a (i32, i32, i32);
    fn set_position(&mut self, position: (i32, i32, i32));

    fn get_facing(&self) -> &'a Facing;

    fn is_solid(&self) -> bool;
    fn is_sticky(&self) -> bool;

    fn get_power(&self) -> (&i8, Option<&'a dyn Block<'a>>);
    fn set_power(&mut self, level: i8, source: Option<&'a dyn Block<'a>>);

    fn get_kind(&self) -> &Kind;

    fn get_hitbox(&self) -> &str;

    fn get_movable(&self) -> Vec<Movable>;
}

pub enum Kind {
    Block,
    BlockEntity(Box<dyn BlockEntity>),
    Component(Component),
}

pub enum Movable {
    SlimePushable,
    PistonPushable,
}

#[derive(Clone)]
pub enum Facing {
    /// -Z
    North,
    /// +Z
    South,
    /// +X
    East,
    /// -X
    West,
    Up,
    Down,
}
