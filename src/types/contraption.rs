use super::{block::Block, compiler::Graph};
use crate::version::Version;

pub type Position = (u16, u16, u16);

pub trait World<'a> {
    fn get_block(&self, pos: Position) -> &'a dyn Block<'a>;
    fn get_block_mut(&mut self, pos: Position) -> &'a mut dyn Block<'a>;

    fn get_has_updated(&self) -> bool;
    fn get_has_state_updated(&self) -> bool;

    fn bounds(&self) -> (Position, Position);
}

/// Modling the blocks supplied for the contraption
/// Warning: You should not supplie the whole world, this will be slow. You should supplie each
/// contraption. This allows for you to use multi-threading
pub struct Contraption<'a, T: World<'a>> {
    world: &'a mut T,
    verson: Version,
    last_graph: Option<Graph>,
}

impl<'a, T: World<'a>> Contraption<'a, T> {
    pub fn new(world: &'a mut T) -> Self {
        Self {
            world,
            verson: Version::default(),
            last_graph: None,
        }
    }

    pub fn get_world(&'a self) -> &'a T {
        self.world
    }

    pub fn get_world_mut(&'a mut self) -> &'a mut T {
        self.world
    }

    /// Get the MC version for the contraption
    pub fn get_version(&self) -> &Version {
        &self.verson
    }

    /// Set the MC version for the contraption
    pub fn set_version(&mut self, version: Version) {
        self.verson = version;
    }

    pub fn get_graph(&self) -> Option<Graph> {
        self.last_graph.clone()
    }

    pub fn has_graph(&self) -> bool {
        self.last_graph.is_some()
    }

    #[allow(clippy::missing_panics_doc)]
    /// To be called each tick
    pub fn tick(&mut self) {
        todo!()
    }
}
