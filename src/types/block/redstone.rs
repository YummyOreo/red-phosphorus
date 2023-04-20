use super::Facing;

pub struct DelayState {
    pub delay: i8,
    pub delay_left: i8,
}

pub enum Component {
    Dust {
        power_direction: Vec<Facing>,
    },
    Block,
    Tourch {
        power_direction: Vec<Facing>,
        burnt_out: bool,
    },
    Repeater {
        delay: DelayState,
        locked: bool,
    },
    Comparator {
        subtract: bool,
        /// Left, Middle (back), Right
        signal_in: (i8, i8, i8),
    },
    Lever,
    /// Redstone buttons
    /// Change the delay of the button based on the type of button
    ///
    /// Wooden: 30 ticks (15 redstone ticks)
    /// Stone: 20 ticks (10 redstone ticks)
    Button {
        delay: DelayState,
    },
    /// Iron and Gold pressure plates
    WeightedPressurePlate(Box<dyn WeightedPressurePlate>),
    /// Wooden pressure plates
    PressurePlate,
    Piston {
        extent_phase: PistonPhase,
    },
    PistonHead,
    StickyPiston {
        extent_phase: PistonPhase,
    },
    SticyPistonHead,
    Observer,
    Lamp,
    TargetBlock {
        delay: DelayState,
    },
    NoteBlock,
    /// All rails that can be activated
    Rail,
    Lecturn,
    DoorUpper,
    DoorLower,
    Trapdoor,
}

pub trait WeightedPressurePlate {
    /// Calculate the signal strength based on number of entities on it
    ///
    /// See `utils` for imlps of this
    fn calc(&self) -> i8;
}

pub enum PistonPhase {
    Retracted,
    Extenting,
    Extended,
    Retracting,
}

pub enum UpdateDirection {
    /// From the source out
    FromSource,
    /// From last to source
    AwaySource,
}

pub mod utils {
    use super::UpdateDirection;
    pub const DUST_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::FromSource;
    pub const RAIL_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::AwaySource;

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
