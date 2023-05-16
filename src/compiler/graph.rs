use crate::types::{
    block::{redstone::Component, Block, Kind},
    compiler::{Node, NodeKind},
    contraption::Position,
};

#[allow(unused)]
/// Just makes the node, does not make the connections
pub fn match_block(block: Option<&Block>, pos: Position) -> Node {
    if let Some(block) = block {
        let is_solid = block.is_solid();
        match block.get_kind() {
            Kind::Block if is_solid => Node::new(NodeKind::Block, pos),
            Kind::Block => Node::new(NodeKind::Air, pos),
            Kind::Component(component) => match_component(component, pos),
        }
    } else {
        Node::new(NodeKind::Air, pos)
    }
}

pub fn match_component(component: &Component, pos: Position) -> Node {
    match component {
        Component::Block => Node::new(NodeKind::PowerSource, pos),
        Component::Dust { power } => Node::new_power(NodeKind::Dust, pos, *power),
        Component::Repeater { delay, locked } => Node::new(
            NodeKind::Repeater {
                delay: *delay,
                locked: *locked,
            },
            pos,
        ),
        Component::Lamp { powered } => {
            if *powered {
                Node::new_power(NodeKind::Lamp, pos, 15)
            } else {
                Node::new(NodeKind::Lamp, pos)
            }
        }
        _ => {
            unimplemented!()
        }
    }
}

macro_rules! check_next_block {
    ($current_block:tt, $bounds:tt, $b:tt) => {{
        let next = $current_block.$b + 1;
        if next >= $bounds.0.$b && next <= $bounds.1.$b {
            let mut new_block = $current_block.clone();
            new_block.$b = next;
            Some(new_block)
        } else {
            None
        }
    }};
}

#[allow(unused)]
pub fn get_next_block(current_block: Position, bounds: (Position, Position)) -> Option<Position> {
    if let Some(block) = check_next_block!(current_block, bounds, 0) {
        Some(block)
    } else if let Some(block) = check_next_block!(current_block, bounds, 1) {
        Some(block)
    } else {
        check_next_block!(current_block, bounds, 2).map(|block| block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_block() {
        let current_block = (0, 0, 0);
        let bounds = ((0, 0, 0), (1, 1, 1));

        assert_eq!((1, 0, 0), get_next_block(current_block, bounds).unwrap());

        let current_block = (1, 0, 0);
        let bounds = ((0, 0, 0), (1, 1, 1));

        assert_eq!((1, 1, 0), get_next_block(current_block, bounds).unwrap());

        let current_block = (1, 1, 0);
        let bounds = ((0, 0, 0), (1, 1, 1));

        assert_eq!((1, 1, 1), get_next_block(current_block, bounds).unwrap());

        let current_block = (1, 1, 1);
        let bounds = ((0, 0, 0), (1, 1, 1));

        assert_eq!(None, get_next_block(current_block, bounds));
    }
}
