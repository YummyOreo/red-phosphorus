use crate::{
    types::{
        compiler::{Graph, State},
        contraption::World,
    },
    utils::compiler::calc_bounds,
};

pub mod single_thread {
    use super::*;

    #[allow(unused)]
    pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
        let bounds = world.bounds();
        let size = calc_bounds(bounds);

        let state = State::new(bounds.0);

        let fist_block = world.get_block(bounds.0);
        todo!()
    }
}
