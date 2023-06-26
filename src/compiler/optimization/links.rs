use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::types::compiler::{Graph, Sources};

// Go through each node, and run dijkstra's algorithm to see if it connects to a source
// if id does not, then that node (and the edges going out of it) is removed
pub fn remove_unused_links(graph: Graph, power_sources: Sources) -> Graph {
    for node_index in graph.node_indices() {
        // do stuff
    }
    todo!()
}
