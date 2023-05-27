use crate::types::{
    block::{redstone::Component, Block, Kind},
    compiler::{Node, NodeKind},
    contraption::Position,
};

/// Just makes the node, does not make the connections
pub fn match_block(block: &Block, pos: Position) -> Option<Node> {
    let is_solid = block.is_solid();
    match block.get_kind() {
        Kind::Block if is_solid => Some(Node::new(NodeKind::Block, pos)),
        Kind::Block => None,
        Kind::Component(component) => Some(match_component(block, component, pos)),
    }
}

pub fn match_component(block: &Block, component: &Component, pos: Position) -> Node {
    match component {
        Component::Block => Node::new_with_power(NodeKind::PowerSource, pos, 15),
        Component::Dust => Node::new_with_power(NodeKind::Dust, pos, block.get_power()),
        Component::Repeater { delay, locked } => Node::new(
            NodeKind::Repeater {
                delay: *delay,
                locked: *locked,
            },
            pos,
        ),
        Component::Lamp => Node::new_with_power(NodeKind::Lamp, pos, block.get_power()),
        _ => {
            unimplemented!()
        }
    }
}
