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
        Kind::Component(component) => Some(match_component(component, pos)),
    }
}

pub fn match_component(component: &Component, pos: Position) -> Node {
    match component {
        Component::Block => Node::new(NodeKind::PowerSource, pos),
        Component::Dust { power } => Node::new_with_power(NodeKind::Dust, pos, *power),
        Component::Repeater { delay, locked } => Node::new(
            NodeKind::Repeater {
                delay: *delay,
                locked: *locked,
            },
            pos,
        ),
        Component::Lamp { powered } => {
            if *powered {
                Node::new_with_power(NodeKind::Lamp, pos, 15)
            } else {
                Node::new(NodeKind::Lamp, pos)
            }
        }
        _ => {
            unimplemented!()
        }
    }
}
