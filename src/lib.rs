pub mod types;
pub mod version;
use types::world::Contraption;

use self::version::Version;

pub struct Redstone<'a> {
    verson: Version,
    world: &'a mut Contraption<'a>,
}

impl<'a> Redstone<'a> {
    pub fn new(world: &'a mut Contraption) -> Self {
        Redstone {
            world,
            verson: Default::default(),
        }
    }

    pub fn get_version(&self) -> &Version {
        &self.verson
    }

    pub fn set_version(&mut self, version: Version) {
        self.verson = version;
    }

    pub fn tick(&mut self) {
        todo!()
    }

    pub fn set_world(&mut self, world: &'a mut Contraption<'a>) {
        self.world = world;
    }
}
