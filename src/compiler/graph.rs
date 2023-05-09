use crate::{
    types::{
        compiler::{Graph, State},
        contraption::{Position, World},
    },
    utils::compiler::calc_bounds,
};

#[allow(unused)]
pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
    let bounds = world.bounds();
    let size = calc_bounds(bounds);

    let state = State::new(bounds.0);

    todo!()
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
