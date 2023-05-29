use crate::types::{
    block::{Block, Kind},
    compiler::{Node, NodeKind},
};

pub fn match_block(block: &Block) -> Option<Node> {
    let pos = block.get_position();
    let power = block.get_power();
    match block.get_kind() {
        Kind::Block if block.get_solid() => Some(Node::new_with_power(pos, NodeKind::Solid, power)),
        Kind::Block => None,
        Kind::Component(component) => todo!(),
    }
}
