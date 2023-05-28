#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate
)]

pub mod block;
pub mod contraption;

pub type PowerLevel = i8;
