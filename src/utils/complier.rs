use crate::types::block::Facing;

pub fn calc_block_update_order(facing: Facing) -> u8 {
    match facing {
        Facing::NegativeX => 0,
        Facing::PositiveX => 1,
        Facing::NegativeY => 2,
        Facing::PositiveY => 3,
        Facing::NegativeZ => 4,
        Facing::PositiveZ => 5,
    }
}
