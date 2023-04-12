use super::block::Block;

pub trait World {
    fn get_block(&mut self, position: &(i32, i32, i32)) -> &mut dyn Block;
}
