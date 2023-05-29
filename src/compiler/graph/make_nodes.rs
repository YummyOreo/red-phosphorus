use crate::types::{
    block::{redstone::Component, Block, Kind},
    compiler::{Node, NodeKind},
    contraption::Position,
    PowerLevel,
};

pub fn match_block(block: &Block) -> Option<Node> {
    let pos = block.get_position();
    let power = block.get_power();
    match block.get_kind() {
        Kind::Block if block.get_solid() => Some(Node::new_with_power(pos, NodeKind::Solid, power)),
        Kind::Block => None,
        Kind::Component(component) => Some(match_component(component, pos, power)),
    }
}

pub fn match_component(component: &Component, pos: Position, power: PowerLevel) -> Node {
    match component {
        Component::Dust => Node::new_with_power(pos, NodeKind::Dust, power),
        Component::Block => Node::new_with_power(pos, NodeKind::PowerSource, 15),
        Component::Lamp => Node::new_with_power(pos, NodeKind::Lamp, power),
        Component::Lever { flicked } => {
            Node::new_with_power(pos, NodeKind::ToggleablePowerSource { on: *flicked }, 15)
        }
        Component::Tourch { on } => {
            Node::new_with_power(pos, NodeKind::ToggleablePowerSource { on: *on }, 15)
        }
        Component::Repeater { delay, locked } => Node::new_with_power(
            pos,
            NodeKind::Repeater {
                delay: *delay,
                locked: *locked,
            },
            power,
        ),
        _ => unimplemented!(),
    }
}
