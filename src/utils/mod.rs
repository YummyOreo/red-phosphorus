#![allow(dead_code, unused)]
pub mod blockentity;
pub mod components;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub use tests::compiler as compiler_test;
