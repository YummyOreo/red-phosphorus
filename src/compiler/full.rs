use crate::{
    types::{
        compiler::{Graph, State},
        contraption::World,
    },
    utils::compiler::calc_bounds,
};

pub mod single_thread {
    use super::*;
    use crate::{compiler::graph::match_block, types::compiler::Node};

    fn make_first_node<'a, W: World<'a>>(world: &'a W) -> Option<Node> {
        let blocks = world.get_blocks((0, 0, 0));
        let mut node = None;
        for pos in blocks {
            let block = world.get_block(pos);
            match block {
                None => continue,
                Some(block) => {
                    if let Some(n) = match_block(block, pos) {
                        node = Some(n);
                        break;
                    }
                }
            }
        }
        node
    }

    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Option<Graph> {
        let bounds = world.bounds();
        let _size = calc_bounds(bounds);

        let mut state = State::new(bounds.0);

        state.graph = Some(Graph::new(make_first_node(world)?));

        let _blocks = world.get_blocks(state.graph.expect("Should be there").root.borrow().pos);
        todo!()
    }

    #[cfg(test)]
    mod test {
        use std::collections::HashMap;

        use super::*;
        use crate::{
            types::block::{Block, Kind},
            utils::test::FakeWorld,
        };

        #[test]
        fn test_first_node() {
            let mut blocks = HashMap::new();
            blocks.insert((1, 1, 1), Block::new((1, 1, 1), Kind::Block));
            blocks.get(&(1, 1, 1)).unwrap();
            let world = FakeWorld {
                bounds: ((0, 0, 0), (10, 10, 10)),
                blocks,
            };

            let expected_node = Node::new(crate::types::compiler::NodeKind::Block, (1, 1, 1));
            assert_eq!(Some(expected_node), make_first_node(&world));

            let mut blocks = HashMap::new();
            blocks.insert(
                (6, 2, 10),
                Block::new(
                    (6, 2, 10),
                    Kind::Component(crate::types::block::redstone::Component::Dust { power: 10 }),
                ),
            );
            blocks.get(&(6, 2, 10)).unwrap();
            let world = FakeWorld {
                bounds: ((0, 0, 0), (10, 10, 10)),
                blocks,
            };

            let mut expected_node = Node::new(crate::types::compiler::NodeKind::Dust, (6, 2, 10));
            expected_node.power = 10;
            assert_eq!(Some(expected_node), make_first_node(&world));
        }
    }
}
