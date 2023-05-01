use crate::{types::contraption::World, Contraption};

pub fn complie<'a, T: World<'a>>(_contraption: &'a mut Contraption<'a, T>) {}
