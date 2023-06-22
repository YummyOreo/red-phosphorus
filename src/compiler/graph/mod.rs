mod link_nodes;
mod make_nodes;

pub mod single_threaded {
    use crate::types::{compiler::Graph, contraption::World};
    use super::{*, link_nodes::link_nodes};

    pub fn create_graph<'a, W: World<'a>>(world: &'a W, cache: &mut mini_moka::sync::Cache<u64, crate::types::compiler::Node>) -> Graph {
        let graph = make_nodes::make_nodes(world, cache);
        link_nodes(graph, world)
    }
}
