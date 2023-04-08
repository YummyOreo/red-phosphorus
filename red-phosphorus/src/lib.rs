use phos_version::Version;

#[derive(Default)]
pub struct Redstone {
    verson: Version,
}

impl Redstone {
    pub fn new() -> Self {
        Redstone::default()
    }

    pub fn get_version(&self) -> &Version {
        &self.verson
    }

    pub fn set_version(&mut self, version: Version) {
        self.verson = version;
    }
}
