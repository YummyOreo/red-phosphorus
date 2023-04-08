use crate::block::Block;

pub trait World {
    fn get_block(&self, position: &(i32, i32, i32)) -> &dyn Block;
}
