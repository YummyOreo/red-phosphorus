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

    #[allow(unused)]
    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
        let bounds = world.bounds();
        let size = calc_bounds(bounds);

        let mut state = State::new(bounds.0);

        let fist_block = world.get_block(bounds.0);
        let first_node = match_block(fist_block, bounds.0);

        state.graph = Some(Graph::new(first_node));

        todo!()
    }
}
