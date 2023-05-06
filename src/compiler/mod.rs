use crate::{
    types::{compiler::State, contraption::World},
    Contraption,
};

pub fn complie<'a, T: World<'a>>(_contraption: &'a mut Contraption<'a, T>) {
    let mut _state = State::new();
}
