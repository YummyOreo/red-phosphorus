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
    use super::*;
    use crate::{
        types::compiler::{Graph, Link, Node, NodeKind},
        utils::test::{graph::graph_eq, make_node},
    };

    #[test]
    fn simple_test() {
        let mut simple_graph = Graph::new();
        let source = simple_graph.add_node(make_node!(pos: (0, 0, 0)));
        let link_to_source = [
            simple_graph.add_node(make_node!(pos: (0, 1, 0))),
            simple_graph.add_node(make_node!(pos: (0, 2, 0))),
            simple_graph.add_node(make_node!(pos: (0, 3, 0))),
        ];
        for node in link_to_source {
            simple_graph.add_edge(source, node, Link::new_power());
        }

        let node_not_reachable = simple_graph.add_node(make_node!(pos: (1, 0, 0)));
        simple_graph.add_edge(
            node_not_reachable,
            source,
            crate::types::compiler::Link::StrongPower,
        );

        let res = remove_unused_links(simple_graph.clone(), vec![source]);

        simple_graph.remove_node(node_not_reachable);

        assert!(graph_eq(
            &res.try_into().unwrap(),
            &simple_graph.try_into().unwrap()
        ));
    }

    #[test]
    fn test_1() {
        let mut graph = Graph::new();
        // Source:
        // Lever --> Block --> Dust --> Dust --> Dust --> Lamp <-- Repeater <--  Block
        // Not connected to source, so should be removed
        // Lamp -- Dust -- Block

        // Make the source
        let source = graph.add_node(make_node!(kind: NodeKind::Lever { on: false }));

        // Make the rest of the graph
        let link_to_prev = [
            source,
            graph.add_node(make_node!(kind: NodeKind::Solid { strongly_power: false })),
            graph.add_node(make_node!(kind: NodeKind::Dust)),
            graph.add_node(make_node!(kind: NodeKind::Dust)),
            graph.add_node(make_node!(kind: NodeKind::Dust)),
            graph.add_node(make_node!(kind: NodeKind::Lamp)),
        ];
        for (pos, nx) in link_to_prev.iter().enumerate() {
            let Some(last) = pos.checked_sub(1) else {
                continue;
            };
            let last = link_to_prev[last];
            graph.add_edge(last, *nx, Link::new_power());
        }

        let link_to_last_source = [
            graph.add_node(make_node!(kind: NodeKind::Repeater { delay: 1, locked: false })),
            graph.add_node(make_node!(kind: NodeKind::Solid { strongly_power: false })),
        ];

        for (pos, nx) in link_to_last_source.iter().enumerate() {
            let last = match pos.checked_sub(1) {
                Some(i) => link_to_last_source[i],
                None => *link_to_prev.last().expect("Should have a last"),
            };
            graph.add_edge(last, *nx, Link::new_power());
        }

        let link_not_connected = [
            graph.add_node(make_node!(kind: NodeKind::Lamp)),
            graph.add_node(make_node!(kind: NodeKind::Dust)),
            graph.add_node(make_node!(kind: NodeKind::Solid { strongly_power: false })),
        ];
        for (pos, nx) in link_not_connected.iter().enumerate() {
            let Some(last) = pos.checked_sub(1) else {
                continue;
            };
            let last = link_not_connected[last];
            graph.add_edge(last, *nx, Link::new_power());
        }

        let res = remove_unused_links(graph.clone(), vec![source]);
        // remove unused links
        for nx in link_not_connected {
            graph.remove_node(nx);
        }

        assert!(graph_eq(&res.into(), &graph.into()));
    }
}
