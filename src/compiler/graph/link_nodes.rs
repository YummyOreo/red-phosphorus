// TODO: remove this
#![allow(unused, dead_code)]
use paste::paste;

use crate::types::{
    block::{redstone::Component, Block, Facing, Kind},
    compiler::{Graph, Link},
    contraption::{Position, World},
};

pub fn link_nodes<'a, W: World<'a>>(graph: Graph, world: &'a W) -> Graph {
    let indices = graph.node_indices();

    for index in indices {
        let node = graph.node_weight(index).expect("node should exist");
        let block = world.get_block(node.pos).expect("block should exist");

        let sources = match block.get_kind() {
            Kind::Block => get_sources(block, world, block::get_adjacent_source),
            Kind::Component(Component::Lamp) => {
                get_sources(block, world, lamp::get_adjacent_source)
            }
            _ => vec![],
        };
    }
    todo!()
}

pub fn get_sources<'a, W: World<'a>>(
    block: &'a Block,
    world: &'a W,
    check_block_source: impl Fn(&'a Block, &'a Block) -> Option<(Position, Link)>,
) -> Vec<(Position, Link)> {
    let pos = block.get_position();

    let mut sources = vec![];

    let adjacent_blocks = [
        (pos.0 - 1, pos.1, pos.2),
        (pos.0 + 1, pos.1, pos.2),
        (pos.0, pos.1 - 1, pos.2),
        (pos.0, pos.1 + 1, pos.2),
        (pos.0, pos.1, pos.2 - 1),
        (pos.0, pos.1, pos.2 + 1),
    ];
    for position in adjacent_blocks {
        if let Some(adjacent_block) = world.get_block(position) {
            if let Some(source) = check_block_source(block, adjacent_block) {
                sources.push(source);
            }
        };
    }

    sources
}

mod block {
    use super::*;

    pub fn get_adjacent_source(
        current_block: &Block,
        adjacent_block: &Block,
    ) -> Option<(Position, Link)> {
        let required_facing =
            utils::get_facing(current_block.get_position(), adjacent_block.get_position())
                .expect("should be a adjacent block");
        match adjacent_block.get_kind() {
            Kind::Component(Component::Dust)
                if adjacent_block.get_facing().contains(&required_facing)
                    || required_facing == Facing::NegativeY =>
            {
                Some(Link::new_weak())
            }
            Kind::Component(Component::Repeater {
                delay,
                locked,
                powered,
            }) if adjacent_block.get_facing().contains(&required_facing) => Some(Link::StrongPower),
            Kind::Component(Component::Lever { on })
                if adjacent_block.get_facing().contains(&required_facing) =>
            {
                Some(Link::StrongPower)
            }
            Kind::Component(Component::Tourch { lit }) if required_facing == Facing::PositiveY => {
                Some(Link::StrongPower)
            }
            _ => None,
        }
        .map(|l| (adjacent_block.get_position(), l))
    }
}

mod lamp {
    use super::*;

    pub fn get_adjacent_source(
        current_block: &Block,
        adjacent_block: &Block,
    ) -> Option<(Position, Link)> {
        let required_facing =
            utils::get_facing(current_block.get_position(), adjacent_block.get_position())
                .expect("should be a adjacent block");
        match adjacent_block.get_kind() {
            Kind::Block | Kind::Component(Component::Lamp) => Some(Link::new_weak()),
            Kind::Component(Component::Block) => Some(Link::new_weak()),
            Kind::Component(Component::Dust)
                if adjacent_block.get_facing().contains(&required_facing)
                    || required_facing == Facing::NegativeY =>
            {
                Some(Link::new_weak())
            }
            Kind::Component(Component::Repeater {
                delay,
                locked,
                powered,
            }) if adjacent_block.get_facing().contains(&required_facing) => Some(Link::StrongPower),
            Kind::Component(Component::Lever { on })
                if adjacent_block.get_facing().contains(&required_facing) =>
            {
                Some(Link::StrongPower)
            }
            Kind::Component(Component::Tourch { lit }) if required_facing != Facing::NegativeY => {
                Some(Link::StrongPower)
            }
            _ => None,
        }
        .map(|l| (adjacent_block.get_position(), l))
    }
}

mod dust {
    use super::*;

    pub fn get_adjacent_source(
        current_block: &Block,
        adjacent_block: &Block,
    ) -> Option<(Position, Link)> {
        let required_facing =
            utils::get_facing(current_block.get_position(), adjacent_block.get_position())
                .expect("should be a adjacent block");
        match adjacent_block.get_kind() {
            Kind::Block => Some(Link::new_weak()),
            _ => None,
        }
        .map(|l| (adjacent_block.get_position(), l))
    }
}

mod utils {
    use super::*;
    use crate::types::block::Facing;

    macro_rules! get_facing_macro {
        ($num:expr, $direction:ident, $diff:expr) => {
            paste! {
                match $diff[$num] {
                    1 => Some(Facing::[<Positive $direction>]),
                    -1 => Some(Facing::[<Negative $direction>]),
                    0 => None,
                    _ => unreachable!(),
                }
            }
        };
    }

    /// Gets the way a component needs to face based on block positions
    pub fn get_facing(cb: Position, ob: Position) -> Option<Facing> {
        let diff = vec![cb.0 - ob.0, cb.1 - ob.1, cb.2 - ob.2];
        Some(match get_facing_macro!(0, X, diff) {
            Some(x) => x,
            None => match get_facing_macro!(1, Y, diff) {
                Some(y) => y,
                None => get_facing_macro!(2, Z, diff)?,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;
    use crate::{types::block::Facing, utils::test::*};

    #[test_case((0, 0, 0), (0, 0, 1), Facing::NegativeZ ; "facing neg z")]
    #[test_case((0, 0, 1), (0, 0, 0), Facing::PositiveZ ; "facing pos z")]
    #[test_case((0, 1, 0), (0, 0, 0), Facing::PositiveY ; "facing pos y")]
    #[test_case((0, 0, 0), (0, 1, 0), Facing::NegativeY ; "facing neg y")]
    #[test_case((1, 0, 0), (0, 0, 0), Facing::PositiveX ; "facing pos x")]
    #[test_case((0, 0, 0), (1, 0, 0), Facing::NegativeX ; "facing neg x")]
    fn test_util_get_facing(pos1: Position, pos2: Position, facing: Facing) {
        assert_eq!(utils::get_facing(pos1, pos2).unwrap(), facing);
    }

    #[test]
    fn block_adjacent_source() {
        #[rustfmt::skip]
        let checks = [
            // Dust
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, 0, 1), facing: vec![Facing::NegativeZ]), Some(Link::new_weak()), "dust pointing into block"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, 0, 1), facing: vec![Facing::PositiveX]), None, "dust not pointing into block"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, 1, 0)), Some(Link::new_weak()), "dust ontop of block"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, -1, 0)), None, "dust under block"),
            // Repeater
            (&make_block!(kind: Kind::Component(Component::new_repeater()), pos: (0, 0, 1), facing: vec![Facing::NegativeZ]), Some(Link::StrongPower), "repeater pointing into block"),
            (&make_block!(kind: Kind::Component(Component::new_repeater()), pos: (0, 1, 0)), None, "repeater ontop of block"),
            // Lever
            (&make_block!(kind: Kind::Component(Component::Lever { on: false }), pos: (1, 0, 0), facing: vec![Facing::NegativeX]), Some(Link::StrongPower), "lever on block"),
            (&make_block!(kind: Kind::Component(Component::Lever { on: false }), pos: (1, 0, 0), facing: vec![Facing::PositiveX]), None, "lever not on block"),
            // Tourch
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, -1, 0)),  Some(Link::StrongPower), "tourch under block"),
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, 1, 0), facing: vec![Facing::NegativeY]),  None, "tourch ontop block"),
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, 0, 1), facing: vec![Facing::PositiveZ]),  None, "tourch next to block"),
        ];

        for check in checks {
            dbg!(check.2);
            let current_block = &make_block!(kind: Kind::Block, solid: true);
            let mut res_link = block::get_adjacent_source(current_block, check.0);
            // We don't need to check the position, it will always be the adjacent_block
            let res_link = res_link.map(|l| l.1);
            assert_eq!(res_link, check.1)
        }
    }

    #[test]
    fn lamp_adjacent_source() {
        #[rustfmt::skip]
        let checks = [
            // Block/Lamp
            (&make_block!(kind: Kind::Block, pos: (0, 1, 0)), Some(Link::new_weak()), "block ontop of lamp"),
            (&make_block!(kind: Kind::Component(Component::Lamp), pos: (0, 1, 0)), Some(Link::new_weak()), "lamp ontop of lamp"),
            // Dust
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, 1, 0)), Some(Link::new_weak()), "dust ontop of lamp"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 0), facing: vec![Facing::NegativeX]), Some(Link::new_weak()), "dust pointing into lamp"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 0)), None, "dust not pointing into lamp"),
            // Redstone Block
            (&make_block!(kind: Kind::Component(Component::Block), pos: (0, 1, 0)), Some(Link::new_weak()), "redstoneblock ontop of lamp"),
            // Repeater
            (&make_block!(kind: Kind::Component(Component::new_repeater()), pos: (0, 1, 0)), None, "repeater ontop of lamp"),
            (&make_block!(kind: Kind::Component(Component::new_repeater()), pos: (1, 0, 0), facing: vec![Facing::NegativeX]), Some(Link::StrongPower), "repeater facing into lamp"),
            // Lever
            (&make_block!(kind: Kind::Component(Component::Lever { on: false }), pos: (1, 0, 0), facing: vec![Facing::NegativeX]), Some(Link::StrongPower), "lever on lamp"),
            (&make_block!(kind: Kind::Component(Component::Lever { on: false }), pos: (1, 0, 0), facing: vec![Facing::PositiveX]), None, "lever not on lamp"),
            // Tourch
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, -1, 0)),  Some(Link::StrongPower), "tourch under lamp"),
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, 1, 0), facing: vec![Facing::NegativeY]),  None, "tourch ontop lamp"),
            (&make_block!(kind: Kind::Component(Component::Tourch { lit: false }), pos: (0, 0, 1), facing: vec![Facing::PositiveZ]),  Some(Link::StrongPower), "tourch next to lamp"),
        ];

        for check in checks {
            dbg!(check.2);
            let current_block = &make_block!(kind: Kind::Component(Component::Lamp), solid: true);
            let mut res_link = lamp::get_adjacent_source(current_block, check.0);
            // We don't need to check the position, it will always be the adjacent_block
            let res_link = res_link.map(|l| l.1);
            assert_eq!(res_link, check.1)
        }
    }
}
