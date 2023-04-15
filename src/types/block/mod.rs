pub mod entity;
pub mod redstone;

use self::{entity::BlockEntity, redstone::Component};

pub trait Block<'a> {
    fn get_name(&self) -> &'a str;

    fn get_position(&self) -> &'a (i32, i32, i32);
    fn set_position(&mut self, position: (i32, i32, i32));

    fn get_facing(&self) -> Vec<&'a Facing>;

    fn is_solid(&self) -> bool;
    fn is_sticky(&self) -> bool;

    fn get_power(&self) -> (&i8, Option<&'a dyn Block<'a>>);
    fn get_power_mut(&mut self) -> (&i8, Option<&'a mut dyn Block<'a>>);
    fn set_power(&mut self, level: i8, source: Option<&'a dyn Block<'a>>);

    fn get_kind(&self) -> &Kind;
    fn get_kind_mut(&mut self) -> &mut Kind;

    fn get_hitbox(&self) -> &str;

    fn get_movable(&self) -> Vec<Movable>;
}

pub enum Kind {
    Block,
    BlockEntity(Box<dyn BlockEntity>),
    Component(Component),
}

#[derive(Clone)]
/// The way that a block is moveable
/// Some blocks can't be moved any ways, some only directly by pistons, and some by both pistons
/// and slime blocks, so this should also be stored in a vector. A empty vectior should be seen as
/// immuvable
///
/// # Examples:
/// ```rust
/// use red_phosphorus::types::block::Movable;
///
/// let fully_movable: Vec<Movable> = vec![Movable::SlimePushable, Movable::PistonPushable];
/// let immovable: Vec<Movable> = vec![];
/// ```
pub enum Movable {
    SlimePushable,
    PistonPushable,
}

#[derive(Clone)]
/// Defining a direction that a block is facing
/// Some blocks may be facing multiple directions (such as rail). So it should be stored in a
/// vector
/// # Examples:
/// ```rust
/// use red_phosphorus::types::block::Facing;
///
/// let facing: Vec<Facing> = vec![Facing::NegativeY];
/// let facing: Vec<Facing> = vec![Facing::NegativeY, Facing::NegativeZ];
/// ```
pub enum Facing {
    NegativeZ,
    PositiveZ,

    PositiveX,
    NegativeX,
    /// Up
    PositiveY,
    /// Down
    NegativeY,
}
