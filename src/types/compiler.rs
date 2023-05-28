use petgraph::stable_graph::StableDiGraph;

use super::{contraption::Position, PowerLevel};

pub enum NodeKind {}

pub struct Node {
    pos: Position,
    power: PowerLevel,
    kind: NodeKind,
}

pub enum Link {
    // u8 being the distance till the next component
    Power(u8),
}

pub type Graph = StableDiGraph<Node, Link>;
