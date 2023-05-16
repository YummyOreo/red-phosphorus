pub mod entity;
pub mod redstone;

use self::redstone::Component;

#[derive(Clone, Default)]
/// Basic struct for a block
pub struct Block {
    pos: (i32, i32, i32),
    kind: Kind,
    solid: bool,
    facing: Vec<Facing>,
}

impl Block {
    pub fn new(pos: (i32, i32, i32), kind: Kind) -> Self {
        Self {
            pos,
            kind,
            ..Default::default()
        }
    }

    pub fn set_position(&mut self, pos: (i32, i32, i32)) {
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

    pub fn get_position(&self) -> (i32, i32, i32) {
        self.pos
    }
    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }
    pub fn is_solid(&self) -> bool {
        self.solid
    }
    pub fn get_facing(&self) -> &Vec<Facing> {
        &self.facing
    }
}

#[derive(Clone)]
pub enum Kind {
    Block,
    Component(Component),
}

impl Default for Kind {
    fn default() -> Self {
        Self::Block
    }
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
/// let fully_movable: Vec<Movable> = vec![Movable::SlimePullable, Movable::PistonPushable];
/// let immovable: Vec<Movable> = vec![];
/// ```
pub enum Movable {
    /// Can be pulled by slimeblocks
    SlimePullable,
    /// Can be pushed by pistons
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
