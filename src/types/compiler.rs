use std::sync::Arc;

use petgraph::stable_graph::StableDiGraph;

use super::{contraption::Position, PowerLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Solid { strongly_power: bool },

    PowerSource,
    Tourch { lit: bool },
    Lever { on: bool },
    Dust,
    Repeater { delay: i8, locked: bool },
    Lamp,
}

impl Default for NodeKind {
    fn default() -> Self {
        Self::Solid {
            strongly_power: false,
        }
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
    StrongPower,
    // Uses `Arc<[Position]>` because it should be immutable
    Power {
        distance: i8,
        blocks: Arc<[Position]>,
    },
}

impl Link {
    pub fn new_strong() -> Self {
        Self::StrongPower
    }

    pub fn new_weak() -> Self {
        Link::Power {
            distance: 0,
            blocks: vec![].into(),
        }
    }
}

pub type Graph = StableDiGraph<Node, Link>;
