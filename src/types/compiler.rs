pub struct BasicState<'a> {
    pub power: i8,
    pub connections: Vec<&'a mut Tree<'a>>,
}

pub enum Tree<'a> {
    PowerSource {
        basic_state: BasicState<'a>,
    },
    Block {
        basic_state: BasicState<'a>,
    },
    Air,

    Dust {
        basic_state: BasicState<'a>,
    },
    Repeater {
        delay: i8,
        locked: bool,
        basic_state: BasicState<'a>,
    },
    Lamp {
        basic_state: BasicState<'a>,
    },
}

impl<'a> Tree<'a> {
    pub fn get_basic_state(&self) -> Option<&BasicState<'a>> {
        match self {
            Self::Air => None,
            Self::PowerSource { basic_state }
            | Self::Block { basic_state }
            | Self::Dust { basic_state }
            | Self::Repeater { basic_state, .. }
            | Self::Lamp { basic_state } => Some(basic_state),
        }
    }
    pub fn get_basic_state_mut(&mut self) -> Option<&mut BasicState<'a>> {
        match self {
            Self::Air => None,
            Self::PowerSource { basic_state }
            | Self::Block { basic_state }
            | Self::Dust { basic_state }
            | Self::Repeater { basic_state, .. }
            | Self::Lamp { basic_state } => Some(basic_state),
        }
    }
}
