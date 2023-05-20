use crate::{
    types::{
        compiler::{Graph, State},
        contraption::World,
    },
    utils::compiler::calc_bounds,
};

pub mod single_thread {
    use super::*;
    use crate::compiler::graph::match_block;

    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
        let bounds = world.bounds();
        let _size = calc_bounds(bounds);

        let mut state = State::new(bounds.0);

        let blocks = world.get_blocks((-1, 0, 0));
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
