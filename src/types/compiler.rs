use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{contraption::Position, PowerLevel};

#[derive(Clone)]
pub enum NodeKind {
    PowerSource,
    Block,
    Air,

    Dust,
    Repeater { delay: i8, locked: bool },
    Lamp,
}

pub type NodeCell = Rc<RefCell<Node>>;

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub pos: Position,
    pub power: PowerLevel,
    pub edges: Vec<NodeCell>,
}

impl Node {
    pub fn new(kind: NodeKind, pos: Position) -> Self {
        Self::new_power(kind, pos, 0)
    }
    pub fn new_power(kind: NodeKind, pos: Position, power: PowerLevel) -> Self {
        Self {
            kind,
            pos,
            power,
            edges: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Graph {
    pub root: NodeCell,

    pub index: HashMap<Position, NodeCell>,
}

impl Graph {
    pub fn register(&mut self, node: NodeCell, pos: Position) {
        self.index.insert(pos, node);
    }

    pub fn remove(&mut self, pos: Position) -> Option<NodeCell> {
        self.index.remove(&pos)
    }

    pub fn lookup(&self, pos: Position) -> Option<NodeCell> {
        Some(self.index.get(&pos)?.clone())
    }
    pub fn lookup_mut(&mut self, pos: Position) -> Option<NodeCell> {
        Some(self.index.get_mut(&pos)?.clone())
    }
}

#[derive(Default)]
pub struct State {
    pub graph: Option<Graph>,

    pub current_block: Position,
}

impl State {
    pub fn new(start_block: Position) -> Self {
        State {
            current_block: start_block,
            ..Default::default()
        }
    }
}
