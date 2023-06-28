use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::types::compiler::{Graph, Sources};

// Or run a algo that will find every node that can be reached, log that, and remove any that can't
// be reached via a power source
// Then remove the edges that are going out of that node and into that node
pub fn remove_unused_links(graph: Graph, power_sources: Sources) -> Graph {
    for node_index in graph.node_indices() {
        // do stuff
    }
    todo!()
}
