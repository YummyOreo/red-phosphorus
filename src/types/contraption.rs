use super::block::Block;
use crate::version::Version;

/// Modling the blocks supplied for the contraption
/// Warning: You should not supplie the whole world, this will be slow. You should supplie each
/// contraption. This allows for you to use multi-threading
pub struct Contraption<'a> {
    /// The blocks of the redstone contraption that you want to emulate
    blocks: &'a mut Vec<&'a mut Block>,
    verson: Version,
}

impl<'a> Contraption<'a> {
    pub fn new(blocks: &'a mut Vec<&'a mut Block>) -> Self {
        Self {
            blocks,
            verson: Version::default(),
        }
    }

    pub fn get_world(&self) -> &[&'a mut Block] {
        self.blocks
    }
    pub fn get_world_mut(&mut self) -> &mut Vec<&'a mut Block> {
        self.blocks
    }

    /// Get the MC version for the contraption
    pub fn get_version(&self) -> &Version {
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
