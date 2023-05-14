use std::collections::HashMap;

use crate::types::{
    block::{Block, Facing, Kind, Movable},
    contraption::{Position, World},
    PowerLevel,
};

pub mod compiler;

pub struct FakeBlock<'a> {
    pub pos: Position,
    pub name: String,
    pub facing: Vec<Facing>,
    pub solid: bool,
    pub power: PowerLevel,
    pub power_source: Option<&'a dyn Block<'a>>,
    pub movable: Option<Vec<Movable>>,
    kind: Kind,
}

impl<'a> Default for FakeBlock<'a> {
    fn default() -> Self {
        Self {
            pos: (0, 0, 0),
            name: String::new(),
            facing: Vec::default(),
            solid: true,
            power: 0,
            power_source: None,
            movable: None,
            kind: Kind::Block,
        }
    }
}

impl<'a> Block<'a> for FakeBlock<'a> {
    fn update(&self) {}
    fn get_name(&'a self) -> &'a str {
        &self.name
    }
    fn is_solid(&self) -> bool {
        self.solid
    }
    fn get_kind(&'a self) -> &crate::types::block::Kind {
        &self.kind
    }
    fn get_power(&'a self) -> (&PowerLevel, Option<&'a dyn Block<'a>>) {
        (&self.power, self.power_source)
    }
    fn set_power(&mut self, level: PowerLevel, source: Option<&'a dyn Block<'a>>) {
        self.power = level;
        self.power_source = source;
    }
    fn get_facing(&self) -> Vec<Facing> {
        self.facing.clone()
    }
    fn get_movable(&self) -> Option<Vec<Movable>> {
        self.movable.clone()
    }
    fn get_position(&'a self) -> &'a Position {
        &self.pos
    }
    fn set_position(&mut self, position: Position) {
        self.pos = position;
    }
    fn get_kind_mut(&'a mut self) -> &mut crate::types::block::Kind {
        &mut self.kind
    }
}

impl<'a> FakeBlock<'a> {
    pub fn new(pos: Position, solid: bool, power: PowerLevel, facing: Vec<Facing>) -> Self {
        Self {
            pos,
            solid,
            power,
            facing,
            ..Default::default()
        }
    }
}

pub struct FakeWorld<'a> {
    pub blocks: HashMap<Position, Box<dyn Block<'a>>>,
    pub bounds: (Position, Position),

    pub has_updated: bool,
    pub has_state_updated: bool,
}

impl<'a> World<'a> for FakeWorld<'a> {
    fn bounds(&self) -> (Position, Position) {
        self.bounds
    }

    fn get_block(&'a self, pos: Position) -> Option<&'a Box<dyn Block<'a>>> {
        self.blocks.get(&pos)
    }
    fn get_block_mut(&'a mut self, pos: Position) -> Option<&'a mut Box<dyn Block<'a>>> {
        self.blocks.get_mut(&pos)
    }

    fn get_has_updated(&self) -> bool {
        self.has_updated
    }
    fn get_has_state_updated(&self) -> bool {
        self.has_state_updated
    }
}

impl FakeWorld<'static> {
    pub fn new(
        blocks: HashMap<Position, Box<dyn Block<'static>>>,
        bounds: (Position, Position),
    ) -> Self {
        Self {
            blocks,
            bounds,
            has_updated: false,
            has_state_updated: false,
        }
    }
}
