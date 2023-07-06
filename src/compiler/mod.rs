use crate::{
    error::compiler::CompileError,
    types::{
        compiler::{Graph, GraphCache},
        contraption::World,
    },
};
mod graph;
mod optimization;

pub fn complie<'a, W: World<'a>>(
    world: &'a W,
    cache: &mut GraphCache,
) -> Result<Graph, CompileError> {
    if world.get_has_updated() {
        let (graph, sources) = graph::single_threaded::create_graph(world, cache)?;
        let _graph = optimization::optimize(graph, sources);
    }
    todo!()
}
