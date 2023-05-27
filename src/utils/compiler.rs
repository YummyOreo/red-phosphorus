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
}
