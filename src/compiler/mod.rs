use crate::{
    types::{compiler::Graph, contraption::World},
    Contraption,
};

mod graph;

pub fn complie<'a, W: World<'a>>(contraption: &'a mut Contraption<'a, W>) -> Option<Graph> {
    let has_graph = contraption.has_graph();
    let world = contraption.get_world_mut();
    if world.get_has_updated() || !has_graph {
        return Some(graph::full_compile(world));
    } else if world.get_has_state_updated() {
        // update state
        println!("Updating state");
    }
    None
}
