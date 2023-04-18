use super::Facing;

pub enum Component {
    Dust(Dust),
    Block,
    Tourch(Tourch),
    Repeater(Repeater),
    Comparator(Repeater),
    Lever,
    /// Redstone buttons
    /// Change the delay of the button based on the type of button
    ///
    /// Wooden: 30 ticks (15 redstone ticks)
    /// Stone: 20 ticks (10 redstone ticks)
    Button(Button),
    /// Iron and Gold pressure plates
    WeightedPressurePlate(Box<dyn WeightedPressurePlate>),
    /// Wooden pressure plates
    PressurePlate,
    Piston(Piston),
    PistonHead,
    StickyPiston(Piston),
    SticyPistonHead,
    Observer,
    Lamp,
    TargetBlock(TargetBlock),
    NoteBlock,
    /// All rails that can be activated
    Rail(Rail),
    Lecturn(Lecturn),
    DoorUpper,
    DoorLower,
    Trapdoor,
}

pub struct Dust {
    pub power_direction: Vec<Facing>,
}

impl Dust {
    pub const UPDATE_DIRECTION: UpdateDirection = UpdateDirection::FromSource;
}

pub struct Tourch {
    pub power_direction: Vec<Facing>,
    pub is_burnt_out: bool,
}

pub struct Repeater {
    pub delay: i8,
    /// -1 represents not on
    pub delay_left: i8,

    pub locked: bool,
}

pub struct Comparator {
    pub subtract: bool,
    pub signal_in: i8,
    pub signal_left: i8,
    pub signal_right: i8,

    pub output_strength: i8,
}

pub struct Button {
    pub delay: i16,
    /// -1 represents not on
    pub delay_left: i16,
}

pub trait WeightedPressurePlate {
    /// Calculate the signal strength based on number of entities on it
    ///
    /// See `utils` for imlps of this
    fn calc(&self) -> i8;
}

pub struct Piston {
    pub extent_phase: PistonPhase,
}

pub enum PistonPhase {
    Retracted,
    Extenting,
    Extended,
    Retracting,
}

pub struct TargetBlock {
    /// -1 represents not on
    pub delay_left: i16,
    /// Depends what hits it
    pub delay: i16,

    pub caused_by_projectile: bool,

    pub output_strength: i8,
}

pub struct Rail;

impl Rail {
    pub const UPDATE_DIRECTION: UpdateDirection = UpdateDirection::AwaySource;
}

pub struct Lecturn {
    /// -1 represents not on
    pub delay_left: i16,
    /// Depends what hits it
    pub delay: i16,

    pub output_strength: i8,
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
