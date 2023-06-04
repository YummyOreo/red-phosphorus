use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};

use crate::types::{
    block::{redstone::Component, Block, Facing, Kind},
    contraption::{Blocks, Position, World},
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

    pub fn new_random(blocks: Vec<Block>) -> Self {
        let mut blocks_map = HashMap::new();
        let mut smallest = (0, 0, 0);
        let mut largest = (0, 0, 0);
        for block in blocks {
            let pos = block.get_position();
            // TODO: find a better way to do this
            if smallest.0 > pos.0 {
                smallest.0 = pos.0;
            }
            if smallest.1 > pos.1 {
                smallest.1 = pos.1;
            }
            if smallest.2 > pos.2 {
                smallest.2 = pos.2;
            }

            if largest.0 < pos.0 {
                largest.0 = pos.0;
            }
            if largest.1 < pos.1 {
                largest.1 = pos.1;
            }
            if largest.2 < pos.2 {
                largest.2 = pos.2;
            }
            blocks_map.insert(pos, block);
        }

        FakeWorld {
            bounds: random_bounds(smallest, largest),
            blocks: blocks_map,
        }
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
                let mut block = Block::new_simple(Default::default(), Kind::Block);
                block.set_solid(false);
                block
            }
            1 => Block::new_simple(Default::default(), Kind::Block),
            2 => Block::new_simple(Default::default(), Kind::Component(Component::Block)),
            3 => {
                let mut component = Component::Dust;
                Block::new_simple(Default::default(), Kind::Component(component))
            }
            4 => {
                let mut component = Component::Repeater {
                    delay: rng.gen_range(1..=4),
                    locked: false,
                };
                Block::new_simple(Default::default(), Kind::Component(component))
            }
            5 => {
                let mut component = Component::Lamp;
                Block::new_simple(Default::default(), Kind::Component(component))
            }
            _ => unreachable!(),
        }
    }
}

fn random_bounds(min: Position, largest: Position) -> (Position, Position) {
    let mut seeds = (rand::random(), rand::random(), rand::random());
    let mut rng = (
        rand::prelude::StdRng::seed_from_u64(seeds.0),
        rand::prelude::StdRng::seed_from_u64(seeds.1),
        rand::prelude::StdRng::seed_from_u64(seeds.2),
    );
    // So we can repo tests if they fail
    dbg!(seeds);
    let start = min;
    let end = (
        rng.0.gen_range(largest.0..i8::MAX as i32),
        rng.1.gen_range(largest.1..i8::MAX as i32),
        rng.2.gen_range(largest.2..i8::MAX as i32),
    );

    (start, end)
}

pub fn random_world() -> FakeWorld {
    let bounds = random_bounds((0, 0, 0), (i8::MAX as i32, i8::MAX as i32, i8::MAX as i32));
    let start = bounds.0;
    let mut current_block = start;
    current_block.0 -= 1;

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
