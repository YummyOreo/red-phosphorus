pub enum Version {
    #[cfg(feature = "1_19")]
    Verson1_19,
}

impl Default for Version {
    #[cfg(feature = "1_19")]
    fn default() -> Self {
        Self::Verson1_19
    }

    #[not(default)]
    fn default() -> Self {
        panic!("You must have at least one version feature enabled")
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "1_19")]
            Self::Verson1_19 => String::from("1.19.*"),
            _ => panic!("You must have at least one version feature enabled"),
        }
    }
}
