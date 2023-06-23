use mini_moka::sync::Cache;

use crate::types::{
    compiler::{Graph, Node},
    contraption::World,
};
mod graph;

pub fn complie<'a, W: World<'a>>(world: &'a W, cache: &mut Cache<u64, Node>) -> Graph {
    if world.get_has_updated() {
        let _graph = graph::single_threaded::create_graph(world, cache);
    }
    todo!()
}
