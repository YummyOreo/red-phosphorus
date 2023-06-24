use thiserror::Error;

use crate::types::{block::redstone::Component, contraption::Position};

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("the component {0:?} has not been impemented yet")]
    ComponentNotImplemented(Component),
    #[error("the block at the position {0:?} does not exist")]
    BlockDoesNotExist(Position),
    #[error("the block at the position {0:?} is not facing any direction")]
    BlockNotFacingADirection(Position),
}

#[derive(Error, Debug)]
pub enum OptimizationError {}
