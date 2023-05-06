use crate::{
    types::{compiler::State, contraption::World},
    Contraption,
};

pub fn complie<'a, T: World<'a>>(contraption: &'a mut Contraption<'a, T>) {
    let mut state = State::new();

    if contraption.get_world().get_has_updated() || !contraption.has_graph() {
        // Recompiling the graph
        println!("Recompiling");
    } else if contraption.get_world().get_has_state_updated() {
        // update state
        println!("Updating state");
    } else {
        state.graph = Some(contraption.get_graph().unwrap());
    }
}
