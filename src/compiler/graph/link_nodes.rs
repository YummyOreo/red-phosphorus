use crate::types::{
    block::Block,
    compiler::Graph,
    contraption::{Position, World},
};

pub fn link_nodes<'a, W: World<'a>>(graph: Graph, world: &'a W) -> Graph {
    let indices = graph.node_indices();

    for index in indices {
        let node = graph.node_weight(index).expect("node should exist");
        let block = world.get_block(node.pos).expect("block should exist");

        let sources = get_potential_sources(block, world);
    }
    todo!()
}

fn get_potential_sources<'a, W: World<'a>>(block: &Block, world: &'a W) -> Vec<Position> {
    todo!()
}
