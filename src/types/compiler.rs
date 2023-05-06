use std::{cell::RefCell, collections::HashSet, rc::Rc};

use super::{contraption::Position, PowerLevel};

pub enum NodeKind {
    PowerSource,
    Block,
    Air,

    Dust,
    Repeater { delay: i8, locked: bool },
    Lamp,
}

pub type NodeCell = Rc<RefCell<Node>>;

pub struct Node {
    pub kind: NodeKind,
    pub power: PowerLevel,
    pub edges: Vec<NodeCell>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            power: 0,
            edges: Vec::new(),
        }
    }
}
pub struct Graph {
    pub root: NodeCell,

    pub index: HashSet<Position, NodeCell>,
}

#[derive(Default)]
pub struct State {
    pub tree: Option<Graph>,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }
}
