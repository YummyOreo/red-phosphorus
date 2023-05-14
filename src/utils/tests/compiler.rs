use crate::types::{
    block::{Block, Facing, Kind, Movable},
    contraption::Position,
    PowerLevel,
};

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
