const DUST_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::FromSource;
const RAIL_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::AwaySource;

pub enum Component {
    Dust,
    Block,
    Tourch,
    Repeater,
    Comparator,
    Lever,
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
    pub fn get_update_direction(&self) -> Option<UpdateDirection> {
        match self {
            Self::Rail => Some(RAIL_UPDATE_DIRECTION),
            Self::Dust { .. } => Some(DUST_UPDATE_DIRECTION),
            _ => None,
        }
    }
}

pub enum UpdateDirection {
    /// From the source out
    FromSource,
    /// From last to source
    AwaySource,
}
