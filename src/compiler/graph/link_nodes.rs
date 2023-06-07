use petgraph::stable_graph::NodeIndex;

use crate::types::{
    compiler::{Graph, Link, Node},
    contraption::World,
};

fn get_connections<'a, W: World<'a>>(
    node: Node,
    world: &'a W,
    graph: &'a Graph,
) -> Vec<(NodeIndex, Link)> {
    todo!()
}
