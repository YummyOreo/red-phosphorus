use crate::{
    types::{
        compiler::{Graph, State},
        contraption::World,
    },
    utils::compiler::calc_bounds,
};

pub mod single_thread {
    use super::*;
    use crate::{compiler::graph::match_block, utils::compiler::NextBlocks};

    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
        let bounds = world.bounds();
        let _size = calc_bounds(bounds);

        let mut state = State::new(bounds.0);

        let blocks = NextBlocks {
            current_block: (-1, 0, 0),
            bounds,
        };
        for pos in blocks {
            let block = world.get_block(pos);
            if let Some(node) = match_block(block, pos) {
                state.graph = Some(Graph::new(node));
                break;
            }
        }

        todo!()
    }
}
