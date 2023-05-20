use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::types::{
    block::{redstone::Component, Block, Facing, Kind},
    contraption::{Blocks, Position, World},
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

impl Distribution<Facing> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Facing {
        match rng.gen_range(0..6) {
            0 => Facing::NegativeZ,
            1 => Facing::PositiveZ,
            2 => Facing::PositiveX,
            3 => Facing::NegativeX,
            4 => Facing::PositiveY,
            _ => Facing::NegativeY,
        }
    }
}

impl Distribution<Block> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Block {
        match rng.gen_range(0..=5) {
            0 => {
                let mut block = Block::new(Default::default(), Kind::Block);
                block.set_solid(false);
                block
            }
            1 => Block::new(Default::default(), Kind::Block),
            2 => Block::new(Default::default(), Kind::Component(Component::Block)),
            3 => {
                let mut component = Component::Dust { power: 0 };
                Block::new(Default::default(), Kind::Component(component))
            }
            4 => {
                let mut component = Component::Repeater {
                    delay: rng.gen_range(1..=4),
                    locked: false,
                };
                Block::new(Default::default(), Kind::Component(component))
            }
            5 => {
                let mut component = Component::Lamp { powered: false };
                Block::new(Default::default(), Kind::Component(component))
            }
            _ => unreachable!(),
        }
    }
}

pub fn random_world() -> FakeWorld {
    let start = (
        rand::random::<i8>() as i32,
        rand::random::<i8>() as i32,
        rand::random::<i8>() as i32,
    );
    let end = (
        rand::random::<i8>() as i32,
        rand::random::<i8>() as i32,
        rand::random::<i8>() as i32,
    );

    let mut current_block = start;
    current_block.0 -= 1;
    let bounds = (start, end);

    let blocks_pos = Blocks {
        current_block,
        bounds,
    };

    let mut blocks: HashMap<Position, Block> = HashMap::new();

    for pos in blocks_pos {
        let mut block: Block = rand::random();
        if let Kind::Component(Component::Repeater { delay, locked }) = block.get_kind() {
            let facing = loop {
                match rand::random::<Facing>() {
                    // Makes sure that its not facing up
                    Facing::NegativeY => continue,
                    // Makes sure that its not facing up
                    Facing::PositiveY => continue,
                    x => break x,
                }
            };
            block.set_facing(vec![facing]);
        }
        blocks.insert(pos, block);
    }
    FakeWorld { bounds, blocks }
}
