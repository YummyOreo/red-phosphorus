use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    error::compiler::CompileError,
    types::{
        block::{redstone::Component, Block, Kind},
        compiler::{Graph, GraphCache, Node, NodeKind, Sources},
        contraption::{Position, World},
        PowerLevel,
    },
};

pub fn make_nodes<'a, W: World<'a>>(
    world: &'a W,
    cache: &mut GraphCache,
) -> Result<(Graph, Sources), CompileError> {
    let mut graph = Graph::new();

    let mut sources = vec![];

    let mut starting = world.bounds().0;
    starting.0 -= 1;
    for pos in world.get_blocks(starting) {
        if let Some(block) = world.get_block(pos) {
            let mut hasher = DefaultHasher::new();
            block.hash(&mut hasher);
            let hash = hasher.finish();

            if let Some((node, source)) = cache.get(&hash) {
                let index = graph.add_node(node);
                if source {
                    sources.push(index);
                }
                continue;
            }

            if let Some((node, source)) = match_block(block)? {
                cache.insert(hash, (node.clone(), source));
                let index = graph.add_node(node);
                if source {
                    sources.push(index);
                }
            }
        }
    }
    Ok((graph, sources))
}

fn match_block(block: &Block) -> Result<Option<(Node, bool)>, CompileError> {
    let pos = block.get_position();
    let power = block.get_power();
    Ok(match block.get_kind() {
        Kind::Block if block.get_solid() => Some((
            Node::new_with_power(
                pos,
                NodeKind::Solid {
                    strongly_power: false,
                },
                power,
            ),
            false,
        )),
        Kind::Block => None,
        Kind::Component(component) => Some(match_component(component, pos, power)?),
    })
}

fn match_component(
    component: &Component,
    pos: Position,
    power: PowerLevel,
) -> Result<(Node, bool), CompileError> {
    Ok(match component {
        Component::Dust => (Node::new_with_power(pos, NodeKind::Dust, power), false),
        Component::Block => (Node::new_with_power(pos, NodeKind::PowerSource, 15), true),
        Component::Lamp => (Node::new_with_power(pos, NodeKind::Lamp, power), false),
        Component::Lever { on } => (
            Node::new_with_power(pos, NodeKind::Lever { on: *on }, 15),
            true,
        ),
        Component::Tourch { lit } => (
            Node::new_with_power(pos, NodeKind::Tourch { lit: *lit }, 15),
            true,
        ),
        Component::Repeater {
            delay,
            locked,
            powered,
        } => (
            Node::new_with_power(
                pos,
                NodeKind::Repeater {
                    delay: *delay * 2, // Converts redstone ticks to ticks
                    locked: *locked,
                },
                15 * i8::from(*powered),
            ),
            false,
        ),
        _ => return Err(CompileError::ComponentNotImplemented(component.clone())),
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::{
        types::block::{Block, Facing},
        utils::test::{make_block, make_node, BlockBuilder, FakeWorld},
    };

    // Blocks
    #[test_case(make_block!(kind: Kind::Block, solid: true), Some((make_node!(kind: NodeKind::Solid { strongly_power: false }, power: 0), false)) ; "solid block with power 0")]
    #[test_case(make_block!(kind: Kind::Block, solid: true, power: 15), Some((make_node!(kind: NodeKind::Solid { strongly_power: false }, power: 15), false)) ; "solid block with power 15")]
    #[test_case(make_block!(kind: Kind::Block, solid: false, power: 15), None ; "non-solid block with power 15")]
    // Redstone Block
    #[test_case(make_block!(kind: Kind::Component(Component::Block), solid: false), Some((make_node!(kind: NodeKind::PowerSource, power: 15), true)) ; "redstone block")]
    // Lever
    #[test_case(make_block!(kind: Kind::Component(Component::Lever { on: false }), solid: false, power: 15), Some((make_node!(kind: NodeKind::Lever { on: false }, power: 15), true)) ; "lever off")]
    #[test_case(make_block!(kind: Kind::Component(Component::Lever { on: true }), solid: false, power: 0), Some((make_node!(kind: NodeKind::Lever { on: true }, power: 15), true)) ; "lever on")]
    // Tourch
    #[test_case(make_block!(kind: Kind::Component(Component::Tourch { lit: false })), Some((make_node!(kind: NodeKind::Tourch { lit: false }, power: 15), true)) ; "tourch off")]
    #[test_case(make_block!(kind: Kind::Component(Component::Tourch { lit: true })), Some((make_node!(kind: NodeKind::Tourch { lit: true }, power: 15), true)) ; "tourch on")]
    // Repeater
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 1, locked: false, powered: false })), Some((make_node!(kind: NodeKind::Repeater { delay: 2, locked: false }), false)) ; "repeater unlocked default delay")]
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 2, locked: true, powered: true })), Some((make_node!(kind: NodeKind::Repeater { delay: 4, locked: true }, power: 15), false)) ; "repeater locked delay power")]
    #[test_case(make_block!(kind: Kind::Component(Component::Repeater { delay: 3, locked: false, powered: true })), Some((make_node!(kind: NodeKind::Repeater { delay: 6, locked: false }, power: 15), false)) ; "repeater unlocked delay power")]
    // Lamp
    #[test_case(make_block!(kind: Kind::Component(Component::Lamp), power: 1), Some((make_node!(kind: NodeKind::Lamp, power: 1), false)) ; "lamp on")]
    #[test_case(make_block!(kind: Kind::Component(Component::Lamp), power: 0), Some((make_node!(kind: NodeKind::Lamp, power: 0), false)) ; "lamp off")]
    fn test_block_match(block: Block, node: Option<(Node, bool)>) {
        let res = match_block(&block).unwrap();
        assert_eq!(res, node);
    }

    #[test_case(vec![make_block!(kind: Kind::Block, solid: true, pos: (1, 0, 0)),
            make_block!(kind: Kind::Block, solid: true, pos: (0, 0, 2)),
            make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 2), facing: vec![Facing::PositiveX, Facing::NegativeX]),
            make_block!(kind: Kind::Component(Component::Dust), pos: (2, 0, 2), facing: vec![Facing::PositiveX, Facing::NegativeX]),
            make_block!(kind: Kind::Component(Component::Dust), pos: (3, 0, 2), facing: vec![Facing::NegativeZ, Facing::NegativeX]),
            make_block!(kind: Kind::Component(Component::Dust), pos: (3, 0, 0), facing: vec![Facing::NegativeZ, Facing::NegativeX]),
            make_block!(kind: Kind::Component(Component::new_repeater()), pos: (2, 0, 0)),
            make_block!(kind: Kind::Component(Component::Lever { on: false }), pos: (2, 1, 0)),
            make_block!(kind: Kind::Component(Component::Lamp), pos: (0, 0, 1)),
        ] ; "circit 1")
    ]
    #[test_case(vec![make_block!(kind: Kind::Component(Component::Block), pos: (0, 0, 0)),
            make_block!(kind: Kind::Component(Component::new_repeater()), pos: (1, 0, 0), facing: vec![Facing::PositiveX]),
            make_block!(kind: Kind::Component(Component::Dust), pos: (2, 0, 0), facing: vec![Facing::PositiveX, Facing::NegativeX], power: 15),
            make_block!(kind: Kind::Component(Component::Dust), pos: (3, 0, 0), facing: vec![Facing::PositiveX, Facing::NegativeX], power: 14),
            make_block!(kind: Kind::Component(Component::Dust), pos: (4, 0, 0), facing: vec![Facing::PositiveZ, Facing::NegativeX], power: 13),
            make_block!(kind: Kind::Component(Component::Dust), pos: (4, 0, 1), facing: vec![Facing::NegativeZ, Facing::PositiveZ], power: 12),
            make_block!(kind: Kind::Component(Component::Dust), pos: (4, 0, 2), facing: vec![Facing::NegativeZ, Facing::PositiveZ], power: 11),
            make_block!(kind: Kind::Component(Component::Dust), pos: (4, 0, 3), facing: vec![Facing::NegativeZ, Facing::NegativeX], power: 10),
            make_block!(kind: Kind::Component(Component::Dust), pos: (3, 0, 3), facing: vec![Facing::PositiveX, Facing::NegativeX], power: 9),
            make_block!(kind: Kind::Component(Component::Dust), pos: (2, 0, 3), facing: vec![Facing::PositiveX, Facing::NegativeX], power: 8),
            make_block!(kind: Kind::Block, pos: (1, 0, 3), power: 8, solid: true),
            make_block!(kind: Kind::Component(Component::Lamp), pos: (0, 0, 3), power: 8),
        ] ; "circit 2")
    ]
    fn test_make_nodes(mut blocks: Vec<Block>) {
        let world = FakeWorld {
            bounds: ((0, 0, 0), (100, 100, 100)),
            blocks: FakeWorld::vec_to_blocks(blocks.clone()),
        };

        let mut cache = GraphCache::new(10_000);

        let res = make_nodes(&world, &mut cache).unwrap().0;
        for node_i in res.node_indices() {
            let node = res.node_weight(node_i).unwrap();
            let blocks_index = blocks
                .iter()
                .position(|b| b.get_position() == node.pos)
                .unwrap();
            blocks.remove(blocks_index);
        }

        dbg!(&blocks);
        assert!(blocks.is_empty())
    }
}
