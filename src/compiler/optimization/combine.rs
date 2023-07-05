use petgraph::{stable_graph::NodeIndex, visit::Dfs};

use crate::types::compiler::{Graph, Sources, NodeKind};

// Go through and find dust that are connected and that are more than 2 long
// Then add them to a list of list:
// [
// [Dust, Dust, Dust]
// ]
// then combine these into a link between 2 non dust nodes
// If can't make a link, then just link it to a dust at the end
pub fn combine_redstone_dust(mut graph: Graph, sources: Sources) -> Graph {
    let mut dust: Vec<Vec<NodeIndex>> = vec![];
    for source in sources {
        let mut dfs = Dfs::new(&graph, source);
        let mut dusts = vec![];
        while let Some(nx) = dfs.next(&graph) {
            let node = graph.node_weight(nx).unwrap();
            if node.kind == NodeKind::Dust {
                dusts.push(nx);
            }
        }
    }
    todo!()
}
