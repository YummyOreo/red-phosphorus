use petgraph::stable_graph::NodeIndex;

use crate::types::{
    block::{Block, Facing},
    compiler::{Graph, Link, Node},
    contraption::{Position, World},
};

fn get_connections<'a, W: World<'a>>(
    node: Node,
    world: &'a W,
    graph: &'a Graph,
) -> Vec<(NodeIndex, Link)> {
    let block = world
        .get_block(node.pos)
        .expect("all nodes should correspond to a block");
    todo!()
}

fn get_facing_blocks<'a, W: World<'a>>(
    pos: Position,
    facing: Vec<Facing>,
    world: &'a W,
) -> Vec<Option<&Block>> {
    let mut blocks = vec![];
    for side in facing {
        let mut new_pos = pos;
        match side {
            Facing::NegativeZ => new_pos.2 -= 1,
            Facing::PositiveZ => new_pos.2 += 1,
            Facing::NegativeX => new_pos.0 -= 1,
            Facing::PositiveX => new_pos.0 += 1,
            Facing::NegativeY => new_pos.1 -= 1,
            Facing::PositiveY => new_pos.1 += 1,
        }
        blocks.push(world.get_block(new_pos));
    }
    blocks
}
