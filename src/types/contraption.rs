use super::block::Block;
use crate::version::Version;

pub trait World<'a> {
    fn get_block(&'a self, pos: (i8, i8, i8)) -> &'a Block;
    fn get_block_mut(&'a mut self, pos: (i8, i8, i8)) -> &'a mut Block;
}
/// Modling the blocks supplied for the contraption
/// Warning: You should not supplie the whole world, this will be slow. You should supplie each
/// contraption. This allows for you to use multi-threading
pub struct Contraption<'a, T: World<'a>> {
    world: &'a mut T,
    verson: Version,
}

impl<'a, T: World<'a>> Contraption<'a, T> {
    pub fn new(world: &'a mut T) -> Self {
        Self {
            world,
            verson: Version::default(),
        }
    }

    pub fn get_world(&'a self) -> &'a T {
        self.world
    }
    pub fn get_world_mut(&'a mut self) -> &'a mut T {
        self.world
    }

    /// Get the MC version for the contraption
    pub fn get_version(&'a self) -> &'a Version {
        &self.verson
    }

    /// Set the MC version for the contraption
    pub fn set_version(&mut self, version: Version) {
        self.verson = version;
    }

    #[allow(clippy::missing_panics_doc)]
    /// To be called each tick
    pub fn tick(&mut self) {
        todo!()
    }
}
