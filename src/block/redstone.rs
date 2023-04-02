use super::Facing;

pub enum RedstoneComponent {
    Dust(RedstoneDust),
    Block,
    Tourch(RedstoneTourch),
    Repeater(RedstoneRepeater),
    Comparator(RedstoneRepeater),
    Lever,
    Button(RedstoneButton),
    PressurePlate(RedstoneButton),
    Piston(RedstonePiston),
    StickyPiston(RedstonePiston),
    Observer,
    Lamp,
    TargetBlock(RedstoneTargetBlock),
    NoteBlock,
    Rail(RedstoneRail),
    Lecturn(RedstoneLecturn),
    Hopper(RedstoneHopper),
    Door,
    Trapdoor,
}

pub struct RedstoneDust {
    pub power_direction: Vec<Facing>,
}

impl RedstoneDust {
    pub const UPDATE_DIRECTION: UpdateDirection = UpdateDirection::FromSource;
}

pub struct RedstoneTourch {
    pub power_direction: Vec<Facing>,
    pub is_burnt_out: bool,
}

pub struct RedstoneRepeater {
    pub delay: i8,
    /// -1 represents not on
    pub delay_left: i8,

    pub locked: bool,
}

pub struct RedstoneComparator {
    pub subtract: bool,
    pub signal_in: i8,
    pub signal_left: i8,
    pub signal_right: i8,

    pub output_strength: i8,
}

pub struct RedstoneButton {
    pub delay: i16,
    /// -1 represents not on
    pub delay_left: i16,
}

pub struct RedstonePiston {
    pub extent_phase: PistonPhase,
}

pub enum PistonPhase {
    Retracted,
    Extenting,
    Extended,
    Retracting,
}

pub struct RedstoneTargetBlock {
    /// -1 represents not on
    pub delay_left: i16,
    /// Depends what hits it
    pub delay: i16,

    pub caused_by_projectile: bool,

    pub output_strength: i8,
}

pub struct RedstoneRail;

impl RedstoneRail {
    pub const UPDATE_DIRECTION: UpdateDirection = UpdateDirection::AwaySource;
}

pub struct RedstoneLecturn {
    /// -1 represents not on
    pub delay_left: i16,
    /// Depends what hits it
    pub delay: i16,

    pub current_page: i16,
    pub output_strength: i8,
}

pub struct RedstoneHopper {
    pub locked: bool,

    pub output_strength: i8,
}

pub enum UpdateDirection {
    /// From the source out
    FromSource,
    /// From last to source
    AwaySource,
}
