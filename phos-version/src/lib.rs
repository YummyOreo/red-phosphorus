trait GetVersion {
    fn get_version(&self) -> Option<String>;
}

pub enum Version {
    #[cfg(target_feature = "1_19")]
    Verson1_19,
}

impl Default for Version {
    #[cfg(target_feature = "1_19")]
    fn default() -> Self {
        Self::Verson1_19
    }

    fn default() -> Self {
        panic!("You must have at least one version feature enabled")
    }
}

impl GetVersion for Version {
    fn get_version(&self) -> Option<String> {
        match self {
            #[cfg(target_feature = "1_19")]
            Self::Verson1_19 => Some(String::from("1.19.*")),
            _ => None,
        }
    }
}
