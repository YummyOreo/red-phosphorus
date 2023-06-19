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

        let sources = get_potential_sources(block, world);
    }
    todo!()
}

fn get_potential_sources<'a, W: World<'a>>(
    block: &'a Block,
    world: &'a W,
) -> Vec<(Position, Link)> {
    match block.get_kind() {
        Kind::Block => get_sources(block, world, block::get_adjacent_source),
        Kind::Component(Component::Lamp) => get_sources(block, world, lamp::get_adjacent_source),
        _ => vec![],
    }
}

pub fn get_sources<'a, W: World<'a>>(
    block: &'a Block,
    world: &'a W,
    check_block_source: impl Fn(&'a Block, &'a Block) -> Option<(Position, Link)>,
) -> Vec<(Position, Link)> {
    let position = block.get_vec_pos();

    let mut sources = vec![];

    let mut add_state = (0, 1);
    while add_state.0 < 3 {
        if add_state.1 < -1 {
            add_state = (add_state.0 + 1, 1);
        }

        let mut position = position.clone();
        position[add_state.0] += add_state.1;

        if let Some(adjacent_block) = world.get_block((position[0], position[1], position[2])) {
            if let Some(source) = check_block_source(block, adjacent_block) {
                sources.push(source);
            }
        };

        add_state.1 -= 2;
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
            utils::get_facing(current_block.get_vec_pos(), adjacent_block.get_vec_pos())
                .expect("should not be the same block");
        match adjacent_block.get_kind() {
            Kind::Component(Component::Dust) => {
                if adjacent_block.get_facing().contains(&required_facing)
                    || required_facing == Facing::NegativeY
                {
                    return Some((adjacent_block.get_position(), Link::new_weak()));
                }
            }
            Kind::Component(Component::Repeater {
                delay,
                locked,
                powered,
            }) => {
                if adjacent_block.get_facing().contains(&required_facing) {
                    return Some((adjacent_block.get_position(), Link::StrongPower));
                }
            }
            Kind::Component(Component::Lever { on }) => {
                if adjacent_block.get_facing().contains(&required_facing) {
                    return Some((adjacent_block.get_position(), Link::StrongPower));
                }
            }
            Kind::Component(Component::Tourch { lit }) => {
                if required_facing == Facing::PositiveY {
                    return Some((adjacent_block.get_position(), Link::StrongPower));
                }
            }
            _ => {}
        }
        None
    }
}

mod lamp {
    use super::*;

    pub fn get_adjacent_source(
        current_block: &Block,
        adjacent_block: &Block,
    ) -> Option<(Position, Link)> {
        let required_facing =
            utils::get_facing(current_block.get_vec_pos(), adjacent_block.get_vec_pos())
                .expect("should not be the same block");
        match adjacent_block.get_kind() {
            Kind::Block => {
                return Some((adjacent_block.get_position(), Link::new_weak()));
            }
            Kind::Component(Component::Block) => {
                return Some((adjacent_block.get_position(), Link::new_weak()));
            }
            Kind::Component(Component::Dust) => {
                if adjacent_block.get_facing().contains(&required_facing)
                    || required_facing == Facing::NegativeY
                {
                    return Some((adjacent_block.get_position(), Link::new_weak()));
                }
            }
            Kind::Component(Component::Repeater {
                delay,
                locked,
                powered,
            }) => {
                if adjacent_block.get_facing().contains(&required_facing) {
                    return Some((adjacent_block.get_position(), Link::new_weak()));
                }
            }
            Kind::Component(Component::Lever { on }) => {
                if adjacent_block.get_facing().contains(&required_facing) {
                    return Some((adjacent_block.get_position(), Link::new_weak()));
                }
            }
            Kind::Component(Component::Tourch { lit }) => {
                if required_facing != Facing::NegativeY {
                    return Some((adjacent_block.get_position(), Link::new_weak()));
                }
            }
            _ => {}
        }
        None
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
    pub fn get_facing(cb: Vec<i32>, ob: Vec<i32>) -> Option<Facing> {
        let diff = vec![cb[0] - ob[0], cb[1] - ob[1], cb[2] - ob[2]];
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
        let pos1 = vec![pos1.0, pos1.1, pos1.2];
        let pos2 = vec![pos2.0, pos2.1, pos2.2];

        assert_eq!(utils::get_facing(pos1, pos2).unwrap(), facing);
    }

    #[test]
    fn block_ajacent_source() {
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
    fn lamp_ajacent_source() {
        #[rustfmt::skip]
        let checks = [
            // Block
            (&make_block!(kind: Kind::Block, pos: (0, 1, 0)), Some(Link::new_weak()), "block ontop of lamp"),
            // Dust
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (0, 1, 0)), Some(Link::new_weak()), "dust ontop of lamp"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 0), facing: vec![Facing::NegativeX]), Some(Link::new_weak()), "dust pointing into lamp"),
            (&make_block!(kind: Kind::Component(Component::Dust), pos: (1, 0, 0)), None, "dust not pointing into lamp"),
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
