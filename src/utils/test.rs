use std::collections::HashMap;

use crate::types::{
    block::{Block, Facing, Kind},
    contraption::{Position, World},
    PowerLevel,
};

#[derive(Debug, Default)]
pub struct BlockBuilder {
    pub pos: Position,
    pub power: PowerLevel,
    pub kind: Kind,
    pub solid: bool,
    pub facing: Vec<Facing>,
}

impl BlockBuilder {
    pub fn build(&self) -> Block {
        Block::new(
            self.pos,
            self.kind.clone(),
            self.power,
            self.solid,
            self.facing.clone(),
        )
    }

    pub fn set_pos(mut self, position: Position) -> Self {
        self.pos = position;
        self
    }

    pub fn set_power(mut self, power: PowerLevel) -> Self {
        self.power = power;
        self
    }
    pub fn set_kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }
    pub fn set_solid(mut self, solid: bool) -> Self {
        self.solid = solid;
        self
    }
    pub fn set_facing(mut self, facing: Vec<Facing>) -> Self {
        self.facing = facing;
        self
    }
}

macro_rules! make_block {
        ($($b:ident : $t:expr),*) => {
            BlockBuilder {
                $($b : $t),*,
                ..Default::default()
            }.build()
        };
    }

macro_rules! make_node {
        ($($b:ident: $t:expr),*) => {
            Node {
                $($b : $t),*,
                ..Default::default()
            }
        };
    }

pub struct FakeWorld {
    pub bounds: (Position, Position),
    pub blocks: HashMap<Position, Block>,
}

impl FakeWorld {
    pub fn new(blocks: Vec<Block>, bounds: (Position, Position)) -> Self {
        let mut blocks_map = HashMap::new();
        for block in blocks {
            blocks_map.insert(block.get_position(), block);
        }

        FakeWorld {
            bounds,
            blocks: blocks_map,
        }
    }

    pub fn vec_to_blocks(blocks: Vec<Block>) -> HashMap<Position, Block> {
        let mut hblocks = HashMap::new();
        for block in blocks {
            hblocks.insert(block.get_position(), block);
        }
        hblocks
    }
}

impl<'a> World<'a> for FakeWorld {
    fn bounds(&self) -> (Position, Position) {
        self.bounds
    }

    fn get_block(&'a self, pos: Position) -> Option<&'a Block> {
        self.blocks.get(&pos)
    }
    fn get_block_mut(&'a mut self, pos: Position) -> Option<&'a mut Block> {
        self.blocks.get_mut(&pos)
    }

    fn get_has_updated(&self) -> bool {
        false
    }
    fn get_has_state_updated(&self) -> bool {
        false
    }
}

pub(crate) use make_block;
pub(crate) use make_node;
