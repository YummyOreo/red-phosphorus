use std::collections::HashMap;

use crate::types::{
    block::Block,
    contraption::{Position, World},
};

pub struct FakeWorld {
    pub bounds: (Position, Position),
    pub blocks: HashMap<Position, Block>,
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
