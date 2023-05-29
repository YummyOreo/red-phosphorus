use petgraph::stable_graph::StableDiGraph;

use super::{contraption::Position, PowerLevel};

pub enum NodeKind {
    Solid,
}

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

pub enum Link {
    // u8 being the distance till the next component
    Power(u8),
}

pub type Graph = StableDiGraph<Node, Link>;
