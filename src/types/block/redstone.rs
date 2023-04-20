use super::Facing;

const DUST_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::FromSource;
const RAIL_UPDATE_DIRECTION: UpdateDirection = UpdateDirection::AwaySource;

pub struct DelayState {
    max_delay: i8,
    /// If the component is not powered, then this should be set to -1
    delay_left: i8,
}

impl DelayState {
    pub fn new(max_delay: i8) -> Self {
        Self {
            max_delay,
            delay_left: -1,
        }
    }

    pub fn set_delay_left(&mut self, delay: i8) {
        self.delay_left = delay;
    }

    pub fn get_delay(&self) -> i8 {
        self.max_delay
    }

    pub fn get_delay_left(&self) -> Option<i8> {
        if self.delay_left.is_negative() {
            return None;
        }
        Some(self.delay_left)
    }

    /// Will set the delay to the max delay
    pub fn power(&mut self) {
        self.delay_left = self.max_delay;
    }

    /// Will set the delay to off
    pub fn reset_delay(&mut self) {
        self.delay_left = -1;
    }

    /// Will decrement the delay by 1 tick
    /// Will return the ammount of delay left after operations
    ///
    /// If the block is not powered or the delay is not active. Then it will set it to the max
    /// delay
    pub fn decrement_delay(&mut self) -> i8 {
        if self.delay_left.is_negative() || self.delay_left == 0 {
            self.delay_left = self.max_delay + 1;
        }
        self.delay_left -= 1;
        self.delay_left
    }
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

impl Component {
    pub fn get_update_direction(&self) -> Option<UpdateDirection> {
        match self {
            Self::Rail => Some(RAIL_UPDATE_DIRECTION),
            Self::Dust { .. } => Some(DUST_UPDATE_DIRECTION),
            _ => None,
        }
    }
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
