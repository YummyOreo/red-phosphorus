// TODO: remove this
#![allow(unused, dead_code)]

use paste::paste;

use crate::types::{
    block::{redstone::Component, Block, Kind, Facing},
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

                if let Some(compair_block) = world.get_block((position[0], position[1], position[2])) {
                    match compair_block.get_kind() {
                        Kind::Component(Component::Dust) => {
                            if compair_block.get_facing().contains(&utils::get_facing(block.get_vec_pos(), compair_block.get_vec_pos()).unwrap()) {
                                todo!();
                                // soft power
                            } else if utils::get_facing(block.get_vec_pos(), compair_block.get_vec_pos()).unwrap() == Facing::NegativeY {
                                todo!();
                                // soft power
                            }
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

    macro_rules! get_facing_macro {
        ($num:expr, $direction:ident, $diff:expr) => {
            paste! {
                match $diff[$num] {
                    1 => Some(Facing::[<Positive $direction>]),
                    -1 => Some(Facing::[<Negative $direction>]),
                    0 => None,
                    _ => unreachable!(),
                }
            }
        };
    }

    /// Gets the way a component needs to face based on block positions
    pub fn get_facing(cb: Vec<i32>, ob: Vec<i32>) -> Option<Facing> {
        let diff = vec![cb[0] - ob[0], cb[1] - ob[1], cb[2] - ob[2]];
        Some(match get_facing_macro!(0, X, diff) {
            Some(x) => x,
            None => match get_facing_macro!(1, Y, diff) {
                Some(y) => y,
                None => get_facing_macro!(2, Z, diff)?,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::types::block::Facing;

    #[test_case((0, 0, 0), (0, 0, 1), Facing::NegativeZ ; "facing neg z")]
    #[test_case((0, 0, 1), (0, 0, 0), Facing::PositiveZ ; "facing pos z")]
    #[test_case((0, 1, 0), (0, 0, 0), Facing::PositiveY ; "facing pos y")]
    #[test_case((0, 0, 0), (0, 1, 0), Facing::NegativeY ; "facing neg y")]
    #[test_case((1, 0, 0), (0, 0, 0), Facing::PositiveX ; "facing pos x")]
    #[test_case((0, 0, 0), (1, 0, 0), Facing::NegativeX ; "facing neg x")]
    fn test_util_get_facing(pos1: Position, pos2: Position, facing: Facing) {
        let pos1 = vec![pos1.0, pos1.1, pos1.2];
        let pos2 = vec![pos2.0, pos2.1, pos2.2];

        assert_eq!(utils::get_facing(pos1, pos2).unwrap(), facing);
    }
}
