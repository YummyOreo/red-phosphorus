use crate::{
    types::{compiler::Graph, contraption::World},
    utils::compiler::calc_bounds,
};

#[allow(unused)]
pub fn full_compile<'a, W: World<'a>>(world: &'a mut W) -> Graph {
    let bounds = world.bounds();
    let size = calc_bounds(bounds);
    todo!()
}
