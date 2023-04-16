use super::Facing;

pub enum Component {
    /// Redstone dust
    Dust(Dust),
    /// Block of redstone
    Block,
    /// Redstone tourch
    Tourch(Tourch),
    /// Redstone repeater
    Repeater(Repeater),
    /// Redstone comparator
    Comparator(Repeater),
    /// Redstone lever
    Lever,
    /// Redstone buttons
    /// Change the delay of the button based on the type of button
    ///
    /// Wooden: 30 ticks (15 redstone ticks)
    /// Stone: 20 ticks (10 redstone ticks)
    Button(Button),
    // TODO: different types, some are different based on ammount of entities
    PressurePlate(Button),
    Piston(Piston),
    /// Head of piston when extented
    PistonHead,
    StickyPiston(Piston),
    /// A sticky piston head when extended
    SticyPistonHead,
    Observer,
    /// Redstone lamp
    Lamp,
    TargetBlock(TargetBlock),
    NoteBlock,
    /// All rails that can be activated
    Rail(Rail),
    Lecturn(Lecturn),
    /// Top of door
    DoorUpper,
    /// Bottem of door
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
