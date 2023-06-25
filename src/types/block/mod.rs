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
    // -x +x -y +y -z +z
    facing: [bool; 6],
}

impl Block {
    pub fn new_simple(pos: Position, kind: Kind) -> Self {
        Self::new_simple_with_power(pos, kind, 0)
    }

    pub fn new_simple_with_power(pos: Position, kind: Kind, power: PowerLevel) -> Self {
        Self::new(pos, kind, power, true, Default::default())
    }

    pub fn new(
        pos: Position,
        kind: Kind,
        power: PowerLevel,
        solid: bool,
        // -x +x -y +y -z +z
        facing: [bool; 6],
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
    pub fn set_facing(&mut self, facing: [bool; 6]) {
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
    pub fn get_facing(&self) -> &[bool; 6] {
        &self.facing
    }
    pub fn get_power(&self) -> PowerLevel {
        self.power
    }

    pub fn is_facing(&self, facing: &Facing) -> bool {
        self.facing[facing.to_number()]
    }

    pub fn facing_from_vec(facing: Vec<Facing>) -> [bool; 6] {
        let mut f: [bool; 6] = Default::default();
        for face in facing {
            match face {
                Facing::NegativeX => f[0] = true,
                Facing::PositiveX => f[1] = true,
                Facing::NegativeY => f[2] = true,
                Facing::PositiveY => f[3] = true,
                Facing::NegativeZ => f[4] = true,
                Facing::PositiveZ => f[5] = true,
            }
        }
        f
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

impl Facing {
    // Converts it to the number in the list
    pub fn to_number(&self) -> usize {
        match self {
            Self::NegativeX => 0,
            Self::PositiveX => 1,
            Self::NegativeY => 2,
            Self::PositiveY => 3,
            Self::NegativeZ => 4,
            Self::PositiveZ => 5,
        }
    }

    // Converts it from the number in the list
    // # Panics
    // Panics if you provide a number > 5
    pub fn from_number(num: usize) -> Self {
        match num {
            0 => Self::NegativeX,
            1 => Self::PositiveX,
            2 => Self::NegativeY,
            3 => Self::PositiveY,
            4 => Self::NegativeZ,
            5 => Self::PositiveZ,
            _ => unreachable!(),
        }
    }
}
