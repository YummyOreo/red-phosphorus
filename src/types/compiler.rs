use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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
    pub power: i8,
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

    pub index: HashSet<(u32, u32, u32), NodeCell>,
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
