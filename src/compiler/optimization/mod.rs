use crate::types::compiler::{Sources, Graph};

mod links;


pub fn optimize(graph: Graph, sources: Sources) -> Graph {
    links::remove_unused_links(graph, sources)
}
