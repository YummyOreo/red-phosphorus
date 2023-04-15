pub mod entity;
pub mod redstone;

use self::{entity::BlockEntity, redstone::Component};

/// Base trait for a block
pub trait Block<'a> {
    /// Get the name of the block
    fn get_name(&self) -> &'a str;

    /// Get the position of the block in 3d space
    fn get_position(&self) -> &'a (i32, i32, i32);
    /// Set the position of block
    /// used for when a piston is pushing a block
    fn set_position(&mut self, position: (i32, i32, i32));

    /// Get the `Facing` of a block
    /// Sometimes this can be modifired (see rail), but this should be done by you
    ///
    /// See more info in the `Facing` enum
    fn get_facing(&self) -> Vec<Facing>;

    /// Get if the block is solid
    fn is_solid(&self) -> bool;

    /// Get the power level and the block that is powering the block
    fn get_power(&self) -> (&i8, Option<&'a dyn Block<'a>>);
    /// Get the power level and **a mutable refrence to** the block that is powering the block
    fn get_power_mut(&mut self) -> (&i8, Option<&'a mut dyn Block<'a>>);
    /// Set the power level and source
    fn set_power(&mut self, level: i8, source: Option<&'a mut dyn Block<'a>>);

    /// Get the "kind of the block"
    /// See more in the enum `Kind`
    fn get_kind(&self) -> &Kind;
    /// Get the "kind of the block"
    /// Useful for modifying a redstone component
    ///
    /// See more in the enum `Kind`
    fn get_kind_mut(&mut self) -> &mut Kind;

    /// Get if the block is movable and how
    /// If it is immuvable, return `None` or a empty list
    /// See more in the enum `Movable`
    fn get_movable(&self) -> Option<Vec<Movable>>;
}

/// The "kind of the block"
/// A block (in terms of redstone) can be 3 types:
/// - Just a regular block
/// - A block `BlockEntity`
/// - A redstone `Component`
///
/// `Block`: takes nothing
/// `BlockEntity`: Takes a struct implementity the trait `BlockEntity`
/// `Component`: Takes a enum variant from `Component`
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
