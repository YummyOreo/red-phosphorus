// TODO: remove this
#![allow(unused, dead_code)]

use crate::types::{
    block::{redstone::Component, Block, Kind},
    compiler::{Graph, Link},
    contraption::{Position, World},
};

pub fn link_nodes<'a, W: World<'a>>(graph: Graph, world: &'a W) -> Graph {
    let indices = graph.node_indices();

    for index in indices {
        let node = graph.node_weight(index).expect("node should exist");
        let block = world.get_block(node.pos).expect("block should exist");

        let sources = get_potential_sources(block, world);
    }
    todo!()
}

fn get_potential_sources<'a, W: World<'a>>(block: &Block, world: &'a W) -> Vec<(Position, Link)> {
    match block.get_kind() {
        Kind::Block => {
            let position = block.get_vec_pos();
            let mut add_state = (0, 1);
            while add_state.0 < 3 {
                if add_state.1 < -1 {
                    add_state = (add_state.0 + 1, 1);
                }

                let mut position = position.clone();
                position[add_state.0] += add_state.1;

                if let Some(block) = world.get_block((position[0], position[1], position[2])) {
                    match block.get_kind() {
                        Kind::Component(Component::Dust) => {
                            // check if its pointing into the block (if so then its a weak power)
                            // check if its ontop, same as above
                        }
                        Kind::Component(Component::Repeater {
                            delay,
                            locked,
                            powered,
                        }) => {
                            // check if its pointing into the block
                        }
                        Kind::Component(Component::Lever { on }) => {
                            // check if its one block
                        }
                        Kind::Component(Component::Tourch { lit }) => {
                            // check if it is point at block
                        }
                        _ => {}
                    }
                };

                add_state.1 -= 2;
            }
            todo!()
        }
        _ => vec![],
    }
}

mod block {}

mod utils {
    use super::*;
    use crate::types::block::Facing;

    /// Gets the way a component needs to face based on block positions
    fn get_facing(cb: Vec<i32>, ob: Vec<i32>) -> Option<Facing> {
        let diff = vec![cb[0] - ob[0], cb[1] - ob[1], cb[2] - ob[2]];
        // TODO: make it so this is not repeatitive
        match diff[0] {
            1 => {
                return Some(Facing::PositiveX);
            }
            -1 => {
                return Some(Facing::NegativeX);
            }
            0 => {}
            _ => unreachable!(),
        }
        match diff[1] {
            1 => {
                return Some(Facing::PositiveY);
            }
            -1 => {
                return Some(Facing::NegativeY);
            }
            0 => {}
            _ => unreachable!(),
        }
        match diff[2] {
            1 => {
                return Some(Facing::PositiveZ);
            }
            -1 => {
                return Some(Facing::NegativeZ);
            }
            0 => {}
            _ => unreachable!(),
        }
        None
    }
}
