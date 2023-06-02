use crate::types::{
    block::{redstone::Component, Block, Kind},
    compiler::{Graph, Node, NodeKind},
    contraption::{Position, World},
    PowerLevel,
};

pub fn make_nodes<'a, W: World<'a>>(world: &'a W) -> Graph {
    let mut graph = Graph::new();
    for block in world.get_blocks(world.bounds().0) {
        let block = world.get_block(block);
        if let Some(node) = match_block(block) {
            graph.add_node(node);
        }
    }
    graph
}

fn match_block(block: Option<&Block>) -> Option<Node> {
    let block = block?;
    let pos = block.get_position();
    let power = block.get_power();
    match block.get_kind() {
        Kind::Block if block.get_solid() => Some(Node::new_with_power(pos, NodeKind::Solid, power)),
        Kind::Block => None,
        Kind::Component(component) => Some(match_component(component, pos, power)),
    }
}

fn match_component(component: &Component, pos: Position, power: PowerLevel) -> Node {
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
                delay: *delay + 1,
                locked: *locked,
            },
            power,
        ),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::{types::block::Block, utils::test::BlockBuilder};

    macro_rules! make_block {
        ($($b:ident : $t:expr),*) => {
            BlockBuilder {
                $($b : $t),*,
                ..Default::default()
            }.build()
        };
    }

    macro_rules! make_node {
        ($($b:ident: $t:expr),*) => {
            Node {
                $($b : $t),*,
                ..Default::default()
            }
        };
    }

    // Blocks
    #[test_case(make_block!(kind: Kind::Block, solid: true), Some(make_node!(kind: NodeKind::Solid, power: 0)) ; "solid block with power 0")]
    #[test_case(make_block!(kind: Kind::Block, solid: true, power: 15), Some(make_node!(kind: NodeKind::Solid, power: 15)) ; "solid block with power 15")]
    #[test_case(make_block!(kind: Kind::Block, solid: false, power: 15), None ; "non-solid block with power 15")]
    // Redstone Block
    #[test_case(make_block!(kind: Kind::Component(Component::Block), solid: false), Some(make_node!(kind: NodeKind::PowerSource, power: 15)) ; "redstone block")]
    // Lever
    #[test_case(make_block!(kind: Kind::Component(Component::Lever { flicked: false }), solid: false, power: 15), Some(make_node!(kind: NodeKind::ToggleablePowerSource { on: false }, power: 15)) ; "lever off")]
    #[test_case(make_block!(kind: Kind::Component(Component::Lever { flicked: true }), solid: false, power: 0), Some(make_node!(kind: NodeKind::ToggleablePowerSource { on: true }, power: 15)) ; "lever on")]
    // Tourch
    #[test_case(make_block!(kind: Kind::Component(Component::Tourch { on: false })), Some(make_node!(kind: NodeKind::ToggleablePowerSource { on: false }, power: 15)) ; "tourch off")]
    #[test_case(make_block!(kind: Kind::Component(Component::Tourch { on: true })), Some(make_node!(kind: NodeKind::ToggleablePowerSource { on: true }, power: 15)) ; "tourch on")]
    // Repeater
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 0, locked: false })), Some(make_node!(kind: NodeKind::Repeater { delay: 1, locked: false })) ; "repeater unlocked no delay")]
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 2, locked: true }), power: 5), Some(make_node!(kind: NodeKind::Repeater { delay: 3, locked: true }, power: 5)) ; "repeater locked delay power")]
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 3, locked: false }), power: 15), Some(make_node!(kind: NodeKind::Repeater { delay: 4, locked: false }, power: 15)) ; "repeater unlocked delay power")]
    // Lamp
    #[test_case(make_block!(kind: Kind::Component(Component::Lamp), power: 1), Some(make_node!(kind: NodeKind::Lamp, power: 1)) ; "lamp on")]
    #[test_case(make_block!(kind: Kind::Component(Component::Lamp), power: 0), Some(make_node!(kind: NodeKind::Lamp, power: 0)) ; "lamp off")]
    fn test_block_match(block: Block, node: Option<Node>) {
        let res = match_block(Some(&block));
        assert_eq!(res, node);
    }
}
