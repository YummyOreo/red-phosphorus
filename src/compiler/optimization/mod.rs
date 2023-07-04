use crate::types::compiler::{Graph, Sources};

mod links;
mod combine;

pub fn optimize(graph: Graph, sources: Sources) -> Graph {
    combine::combine_redstone_dust(links::remove_unused_links(graph, sources))
}
