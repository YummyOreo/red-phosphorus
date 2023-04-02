use crate::block::Block;

pub trait Component {
    fn get_strength(&self) -> i8;
    fn set_strength(&mut self, strength: i8);
}

pub enum RedstoneComponent<'a> {
    Dust(RedstoneDust<'a>)
}

pub struct RedstoneDust<'a> {
    pub position: (i32, i32),
    pub strength: i8,
    pub source: Option<&'a Block<'a>>,
}
