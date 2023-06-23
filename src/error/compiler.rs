use thiserror::Error;

use crate::types::block::redstone::Component;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("the component {0:?} has not been impemented yet")]
    ComponentNotImplemented(Component),
}

#[derive(Error, Debug)]
pub enum OptimizationError {}
