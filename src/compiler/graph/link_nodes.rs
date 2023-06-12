use crate::types::{
    block::{Block, Kind},
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
    match block.get_kind() {
        Kind::Block => {
            let position = block.get_vec_pos();
            let mut add_state = (0, 1);
            while add_state.0 < 3 {
                if add_state.1 < -1 {
                    add_state = (add_state.0 + 1, 1);
                }

                let mut position = position.clone();
                position[add_state.0] += add_state.1;
                dbg!(position);
                // do stuff
                add_state.1 -= 2;
            }
            todo!()
        }
        _ => vec![],
    }
}
