use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {}

#[derive(Error, Debug)]
pub enum OptimizationError {}
