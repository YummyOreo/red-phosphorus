// TODO: REMOVE
#![allow(unused, dead_code)]
use super::{block::Block, compiler::Graph};
use crate::version::Version;

pub type Position = (i32, i32, i32);

/// A usefull iter that allows you to go through each block in the world
pub struct Blocks {
    pub current_block: Position,
    pub bounds: (Position, Position),
}

macro_rules! check_block {
    ($current_block:expr, $bounds:tt, $b:tt) => {{
        if $current_block.$b == $bounds.1.$b {
            $current_block.$b = $bounds.0.$b;
            true
        } else {
            $current_block.$b += 1;
            false
        }
    }};
}

impl Iterator for Blocks {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        let bounds = self.bounds;
        if check_block!(self.current_block, bounds, 2)
            && check_block!(self.current_block, bounds, 1)
            && check_block!(self.current_block, bounds, 0)
        {
            return None;
        }
        Some(self.current_block)
    }
}

pub trait World<'a> {
    /// Air will be returned as `None`
    fn get_block(&'a self, pos: Position) -> Option<&'a Block>;
    /// Air will be returned as `None`
    fn get_block_mut(&'a mut self, pos: Position) -> Option<&'a mut Block>;

    /// Returns the `Blocks` iter
    fn get_blocks(&self, starting_pos: Position) -> Blocks {
        let bounds = self.bounds();
        Blocks {
            current_block: starting_pos,
            bounds,
        }
    }

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
    graph: Option<Graph>,
}

impl<'a, T: World<'a>> Contraption<'a, T> {
    pub fn new(world: &'a mut T) -> Self {
        Self {
            world,
            verson: Version::default(),
            graph: None,
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

    fn get_graph(&self) -> Option<&Graph> {
        self.graph.as_ref()
    }

    fn has_graph(&self) -> bool {
        self.graph.is_some()
    }
}
