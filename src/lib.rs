pub mod types;
pub mod version;
use types::world::World;

use self::version::Version;

pub struct Redstone<'a> {
    verson: Version,
    world: &'a mut dyn World,
}

impl<'a> Redstone<'a> {
    pub fn new(world: &'a mut dyn World) -> Self {
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

    pub fn set_world(&mut self, world: &'a mut dyn World) {
        self.world = world;
    }
}
