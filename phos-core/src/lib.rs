#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate
)]

use phos_version::Version;

pub mod block;
pub mod utils;
pub mod world;

#[derive(Default)]
pub struct Redstone {
    verson: Version,
}

impl Redstone {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_version(&self) -> &Version {
        &self.verson
    }

    pub fn set_version(&mut self, version: Version) {
        self.verson = version;
    }
}
