use petgraph::stable_graph::NodeIndex;

use crate::types::{
    block::{redstone::Component, Block, Facing, Kind},
    compiler::{Graph, Link, Node, NodeKind},
    contraption::{Position, World},
};

fn get_links<'a, W: World<'a>>(
    node: Node,
    world: &'a W,
    graph: &'a Graph,
) -> Vec<(NodeIndex, Link)> {
    let current_block = world
        .get_block(node.pos)
        .expect("all nodes should correspond to a block");

    let link_check_blocks = get_facing_blocks(current_block.get_position(), current_block.get_facing(), world);

    let mut links = vec![];
    for link_block in link_check_blocks {
        let Some(link_block) = link_block else {continue;};
        if !link_block.get_solid() {
            continue;
        }

        let node_index = graph
            .node_indices()
            .find(|n| {
                graph.node_weight(*n).expect("should be there").pos == link_block.get_position()
            })
            .unwrap();
        links.push((node_index, Link { distance: 0 }))
    }
    links
}

fn get_facing_blocks<'a, W: World<'a>>(
    pos: Position,
    facing: &'a Vec<Facing>,
    world: &'a W,
) -> Vec<Option<&'a Block>> {
    let mut blocks = vec![];
    for side in facing {
        let mut new_pos = pos;
        match side {
            Facing::NegativeZ => new_pos.2 -= 1,
            Facing::PositiveZ => new_pos.2 += 1,
            Facing::NegativeX => new_pos.0 -= 1,
            Facing::PositiveX => new_pos.0 += 1,
            Facing::NegativeY => new_pos.1 -= 1,
            Facing::PositiveY => new_pos.1 += 1,
        }
        blocks.push(world.get_block(new_pos));
    }
    blocks
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::{
        types::block::{redstone::Component, Kind},
        utils::test::*,
    };

    #[test_case(vec![
        make_block!(kind: Kind::Component(Component::Repeater {delay: 1, locked: false, powered: false}), facing: vec![Facing::PositiveX]),
        make_block!(kind: Kind::Block, pos: (1, 0, 0))
    ], 0, vec![1] ; "repeater facing into block")]
    #[test_case(vec![
        make_block!(kind: Kind::Component(Component::Block)),
        make_block!(kind: Kind::Component(Component::Dust), power: 15, facing: vec![Facing::PositiveX, Facing::NegativeX], pos: (1, 0, 0)),
        make_block!(kind: Kind::Block, solid: false, pos: (2, 0, 0)),
    ], 1, vec![0, 2] ; "dust facing into transparent and powersource")]
    #[test_case(vec![
        make_block!(kind: Kind::Component(Component::Block), pos: (0, 0, 1)),
        make_block!(kind: Kind::Component(Component::Block), pos: (1, 0, 0)),
        make_block!(kind: Kind::Component(Component::Block), pos: (1, 0, 2)),
        make_block!(kind: Kind::Component(Component::Block), pos: (2, 0, 1)),
        make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 1), facing: vec![Facing::PositiveX, Facing::PositiveZ, Facing::NegativeX, Facing::NegativeZ]),
    ], 4, vec![0, 1, 2, 3] ; "dust facing 4 powersources")]
    fn test_get_facing_blocks(blocks: Vec<Block>, selected: usize, expect: Vec<usize>) {
        let world = FakeWorld {
            bounds: ((0, 0, 0), (100, 100, 100)),
            blocks: FakeWorld::vec_to_blocks(blocks.clone()),
        };

        let block = blocks.get(selected).unwrap();

        let pos = expect
            .iter()
            .map(|i| blocks.get(*i).unwrap().get_position());

        let blocks: Vec<Option<&Block>> = pos.map(|pos| world.get_block(pos)).collect();

        let mut res = get_facing_blocks(block.get_position(), block.get_facing(), &world);

        dbg!(res.clone());
        dbg!(blocks.clone());
        for block in blocks {
            assert!(res.contains(&block));
            res.remove(res.iter().position(|b| &block == b).unwrap());
        }

        assert!(res.is_empty())
    }
}
