use super::block::Block;

pub struct World<'a> {
    /// The blocks of the redstone contraption that you want to emulate
    blocks: &'a mut Vec<&'a mut dyn Block<'a>>,
}

impl<'a> World<'a> {
    pub fn new(blocks: &'a mut Vec<&'a mut dyn Block<'a>>) -> Self {
        Self { blocks }
    }

    pub fn get_world(&self) -> &[&'a mut dyn Block<'a>] {
        self.blocks
    }
    pub fn get_world_mut(&mut self) -> &mut Vec<&'a mut dyn Block<'a>> {
        self.blocks
    }
}
