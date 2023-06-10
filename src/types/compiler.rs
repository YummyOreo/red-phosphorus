use petgraph::stable_graph::StableDiGraph;

use super::{contraption::Position, PowerLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Solid,

    PowerSource,
    Tourch { lit: bool },
    Lever { on: bool },
    Dust,
    Repeater { delay: i8, locked: bool },
    Lamp,
}

impl Default for NodeKind {
    fn default() -> Self {
        Self::Solid
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Node {
    pub pos: Position,
    pub power: PowerLevel,
    pub kind: NodeKind,
}

impl Node {
    pub fn new(pos: Position, kind: NodeKind) -> Self {
        Self::new_with_power(pos, kind, 0)
    }

    pub fn new_with_power(pos: Position, kind: NodeKind, power: PowerLevel) -> Self {
        Self { pos, power, kind }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Link {
    // u8 being the distance till the next component
    Power(u8),
    WeakPower,
}

pub type Graph = StableDiGraph<Node, Link>;
