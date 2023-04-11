use super::Facing;

pub enum Component {
    Dust(Dust),
    Block,
    Tourch(Tourch),
    Repeater(Repeater),
    Comparator(Repeater),
    Lever,
    Button(Button),
    PressurePlate(Button),
    Piston(Piston),
    PistonHead,
    StickyPiston(Piston),
    SticyPistonHead,
    Observer,
    Lamp,
    TargetBlock(TargetBlock),
    NoteBlock,
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
