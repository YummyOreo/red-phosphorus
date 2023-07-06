#[derive(Clone, Debug, Hash)]
pub enum Component {
    Dust,
    Block,
    /// Facing should be pointing to the wall it is on
    Tourch {
        lit: bool,
    },
    Repeater {
        delay: i8,
        locked: bool,
        powered: bool,
    },
    Comparator,
    /// Facing should be pointing to the wall it is on
    Lever {
        on: bool,
    },
    /// Redstone buttons
    /// Change the delay of the button based on the type of button
    ///
    /// Wooden: 30 ticks (15 redstone ticks)
    /// Stone: 20 ticks (10 redstone ticks)
    Button {
        delay: i8,
    },
    /// Iron and Gold pressure plates
    WeightedPressurePlate,
    /// Wooden pressure plates
    PressurePlate,
    Piston,
    PistonHead,
    StickyPiston,
    SticyPistonHead,
    Observer,
    Lamp,
    TargetBlock,
    NoteBlock,
    /// All rails that can be activated
    Rail,
    Lecturn,
    DoorUpper,
    DoorLower,
    Trapdoor,
}

impl Component {
    pub fn new_repeater() -> Self {
        Self::Repeater {
            delay: 1,
            locked: false,
            powered: false,
        }
    }
}

pub enum UpdateDirection {
    /// From the source out
    FromSource,
    /// From last to source
    AwaySource,
}
