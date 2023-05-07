use std::{cell::RefCell, collections::HashSet, rc::Rc};

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

#[derive(Clone)]
pub struct Graph {
    pub root: NodeCell,

    pub index: HashSet<Position, NodeCell>,
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
