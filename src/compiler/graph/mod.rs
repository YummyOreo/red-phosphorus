mod link_nodes;
mod make_nodes;

pub mod single_threaded {
    use super::{link_nodes::link_nodes, make_nodes::make_nodes};
    use crate::{
        error::compiler::CompileError,
        types::{
            compiler::{Graph, GraphCache, Sources},
            contraption::World,
        },
    };

    pub fn create_graph<'a, W: World<'a>>(
        world: &'a W,
        cache: &mut GraphCache,
    ) -> Result<(Graph, Sources), CompileError> {
        let (graph, sources) = make_nodes(world, cache)?;
        Ok((link_nodes(graph, world)?, sources))
    }
}
