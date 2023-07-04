use crate::types::compiler::{Graph, Sources};

mod links;
mod combine;

pub fn optimize(graph: Graph, sources: Sources) -> Graph {
    links::remove_unused_links(graph, sources)
}
