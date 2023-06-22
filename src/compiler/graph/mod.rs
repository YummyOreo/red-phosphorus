mod link_nodes;
mod make_nodes;

pub mod single_threaded {
    use mini_moka::sync::Cache;

    use super::{link_nodes::link_nodes, *};
    use crate::types::{
        compiler::{Graph, Node},
        contraption::World,
    };

    pub fn create_graph<'a, W: World<'a>>(world: &'a W, cache: &mut Cache<u64, Node>) -> Graph {
        let graph = make_nodes::make_nodes(world, cache);
        link_nodes(graph, world)
    }
}
