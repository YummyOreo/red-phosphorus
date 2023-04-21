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

pub mod utils {
    /// Input # of entities. If none, then just supplie 0
    pub fn calc_gold_plate(entities: i16) -> i8 {
        entities.clamp(0, 15) as i8
    }

    #[allow(clippy::cast_possible_truncation)]
    /// Input # of entities. If none, then just supplie 0
    pub fn calc_iron_plate(entities: i16) -> i8 {
        // Each "range" is (powerlevel) - 10 + 1 .. (powerlevel * 10)

        let remainder = entities % 10;
        // Checks if divisable by 10
        if remainder == 0 {
            // If it is, then we can just return what was in the 10's place
            return (entities / 10).clamp(0, 15) as i8;
        }

        // if not divisable by 10
        // then add 10 to it           ie. 11 + 10 = 21
        // then remove the one's place ie. 21 - 1 = 20
        // then divide by 10           ie. 20 / 10 = 2
        // the product is then clamped to 15, then converted to a i8
        (((entities + 10) - (remainder)) / 10).clamp(0, 15) as i8
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn iron_plate_test() {
            assert_eq!(0, calc_iron_plate(0));
            assert_eq!(1, calc_iron_plate(10));
            assert_eq!(1, calc_iron_plate(1));
            assert_eq!(9, calc_iron_plate(81));
            assert_eq!(4, calc_iron_plate(32));
            assert_eq!(3, calc_iron_plate(24));
            assert_eq!(11, calc_iron_plate(109));
            assert_eq!(15, calc_iron_plate(145));
        }
    }
}
