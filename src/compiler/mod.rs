use crate::{types::contraption::World, Contraption};

pub fn complie<'a, W: World<'a>>(contraption: &'a mut Contraption<'a, W>) {
    let world = contraption.get_world();
    if world.get_has_updated() {
        // Fully recompile
    }
}
