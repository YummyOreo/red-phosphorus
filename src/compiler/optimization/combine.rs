use crate::types::compiler::Graph;

// Go through and find dust that are connected and that are more than 2 long
// Then add them to a list of list:
// [
// [Dust, Dust, Dust]
// ]
// then combine these into a link between 2 non dust nodes
// If can't make a link, then just link it to a dust at the end
pub fn combine_redstone_dust(graph: Graph) -> Graph {
    todo!();
}
