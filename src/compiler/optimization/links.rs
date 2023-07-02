use petgraph::{stable_graph::NodeIndex, visit::Dfs};

use crate::types::compiler::{Graph, Sources};

// Or run a algo that will find every node that can be reached, log that, and remove any that can't
// be reached via a power source
// Then remove the edges that are going out of that node and into that node
pub fn remove_unused_links(mut graph: Graph, power_sources: Sources) -> Graph {
    let keep_nodes = get_reachable_nodes(&graph, power_sources);
    let mut remove_nodes = vec![];
    for nx in graph.node_indices() {
        if !keep_nodes.contains(&nx) {
            remove_nodes.push(nx);
        }
    }

    for nx in remove_nodes {
        graph.remove_node(nx);
    }
    graph
}

fn get_reachable_nodes(graph: &Graph, power_sources: Sources) -> Vec<NodeIndex> {
    let mut keep_nodes: Vec<NodeIndex> = vec![];
    for source in power_sources {
        let mut dfs = Dfs::new(&graph, source);
        while let Some(nx) = dfs.next(&graph) {
            keep_nodes.push(nx);
        }
    }
    keep_nodes
}

#[cfg(test)]
mod tests {
}
