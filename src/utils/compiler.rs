use std::{cell::RefCell, rc::Rc};

use crate::types::{
    compiler::{Node, NodeCell},
    contraption::Position,
};

#[inline]
pub fn make_node(node: Node) -> NodeCell {
    Rc::new(RefCell::new(node))
}

pub fn calc_bounds(bounds: (Position, Position)) -> i32 {
    let (length, width, height) = (
        bounds.1 .0 - bounds.0 .0,
        bounds.1 .1 - bounds.0 .1,
        bounds.1 .2 - bounds.0 .2,
    );

    length.abs() * width.abs() * height.abs()
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

pub fn get_next_block(current_block: Position, bounds: (Position, Position)) -> Option<Position> {
    if let Some(block) = check_next_block!(current_block, bounds, 0) {
        Some(block)
    } else if let Some(block) = check_next_block!(current_block, bounds, 1) {
        Some(block)
    } else {
        check_next_block!(current_block, bounds, 2).map(|block| block)
    }
}

pub struct NextBlocks {
    current_block: Position,
    bounds: (Position, Position),
}

impl Iterator for NextBlocks {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = get_next_block(self.current_block, self.bounds) {
            self.current_block = pos;
            return Some(pos);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bounds() {
        let bounds = ((0, 0, 0), (100, 100, 100));
        assert_eq!(100 * 100 * 100, calc_bounds(bounds));

        let bounds = ((300, 300, 300), (100, 100, 100));
        assert_eq!(200 * 200 * 200, calc_bounds(bounds));

        let bounds = ((1000, 2, 300), (100, 100, 100));
        assert_eq!(900 * 98 * 200, calc_bounds(bounds));
    }

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
