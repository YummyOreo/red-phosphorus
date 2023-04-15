use super::block::Block;

pub trait World {
    /// Get the blocks of the redstone contraption that you want to emulate
    fn get_blocks(&mut self) -> Vec<&mut dyn Block>;
}
