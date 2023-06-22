use mini_moka::sync::Cache;

use crate::types::{compiler::Graph, contraption::World};
mod graph;

pub fn complie<'a, W: World<'a>>(world: &'a W) -> Graph {
    if world.get_has_updated() {
        let mut cache = Cache::new(10_000);
        let _graph = graph::single_threaded::create_graph(world, &mut cache);
    }
    todo!()
}
