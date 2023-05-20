use crate::{
    types::{
        compiler::{Graph, State},
        contraption::World,
    },
    utils::compiler::calc_bounds,
};

pub mod single_thread {
    use super::*;
    use crate::{compiler::graph::match_block, types::compiler::Node};

    fn make_first_node<'a, W: World<'a>>(world: &'a W) -> Option<Node> {
        let blocks = world.get_blocks((-1, 0, 0));
        let mut node = None;
        for pos in blocks {
            let block = world.get_block(pos);
            if let Some(n) = match_block(block, pos) {
                node = Some(n);
                break;
            }
        }
        node
    }

    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Option<Graph> {
        let bounds = world.bounds();
        let _size = calc_bounds(bounds);

        let mut state = State::new(bounds.0);

        match make_first_node(world) {
            Some(node) => state.graph = Some(Graph::new(node)),
            None => {
                // All blocks are air, thus no need for a graph
                return None;
            }
        }

        let _blocks = world.get_blocks(state.graph.expect("Should be there").root.borrow().pos);
        todo!()
    }
}
