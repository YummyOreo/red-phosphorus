pub mod redstone;

use self::redstone::Component;
use super::{contraption::Position, PowerLevel};

#[derive(Clone, Default, Debug, Hash)]
/// Basic struct for a block
pub struct Block {
    pos: Position,
    power: PowerLevel,
    kind: Kind,
    solid: bool,
    facing: Vec<Facing>,
}

impl Block {
    pub fn new_simple(pos: Position, kind: Kind) -> Self {
        Self::new_simple_with_power(pos, kind, 0)
    }

    pub fn new_simple_with_power(pos: Position, kind: Kind, power: PowerLevel) -> Self {
        Self::new(pos, kind, power, true, vec![])
    }

    pub fn new(
        pos: Position,
        kind: Kind,
        power: PowerLevel,
        solid: bool,
        facing: Vec<Facing>,
    ) -> Self {
        Self {
            pos,
            power,
            kind,
            solid,
            facing,
        }
    }

    pub fn set_position(&mut self, pos: Position) {
        self.pos = pos;
    }
    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }
    pub fn set_solid(&mut self, solid: bool) {
        self.solid = solid;
    }
    pub fn set_facing(&mut self, facing: Vec<Facing>) {
        self.facing = facing;
    }
    pub fn set_power(&mut self, power: PowerLevel) {
        self.power = power;
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }
    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }
    pub fn get_solid(&self) -> bool {
        self.solid
    }
    pub fn get_facing(&self) -> &Vec<Facing> {
        &self.facing
    }
    pub fn get_power(&self) -> PowerLevel {
        self.power
    }
}

#[derive(Clone, Debug, Hash)]
pub enum Kind {
    Block,
    Component(Component),
}

impl Default for Kind {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Clone, Debug, Hash)]
/// The way that a block is moveable
/// Some blocks can't be moved any ways, some only directly by pistons, and some by both pistons
/// and slime blocks, so this should also be stored in a vector. A empty vectior should be seen as
/// immuvable
///
/// # Examples:
/// ```rust
/// use red_phosphorus::types::block::Movable;
///
/// let fully_movable: Vec<Movable> = vec![Movable::SlimePullable, Movable::PistonPushable];
/// let immovable: Vec<Movable> = vec![];
/// ```
pub enum Movable {
    /// Can be pulled by slimeblocks
    SlimePullable,
    /// Can be pushed by pistons
    PistonPushable,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
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
