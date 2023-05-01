pub enum Version {
    #[cfg(feature = "1_19")]
    Verson1_19,
}

impl Default for Version {
    cfg_if::cfg_if! {
        if #[cfg(feature = "1_19")] {
            #[inline(always)]
            fn default() -> Self {
                Self::Verson1_19
            }
       } else {
            compile_error!("You must have at least one feature enabled");
       }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            #[cfg(feature = "1_19")]
            Self::Verson1_19 => String::from("1.19.*"),
        }
    }
}
