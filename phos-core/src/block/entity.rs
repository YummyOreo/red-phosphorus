#[allow(clippy::module_name_repetitions)]
pub trait BlockEntity {
    fn set_slot(&mut self, slot_name: SlotName, slot_content: Option<Slot>);
    fn get_slot(&self, slot_name: SlotName) -> Option<Slot>;

    fn get_all(&self) -> Vec<(SlotName, Option<Slot>)>;

    fn get_signal_strength(&self) -> i8;
}

pub enum SlotName {
    Num(i32),
    Name(String),
}

pub struct Slot {
    pub item: (String, ItemType),
    pub ammount: i32,
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            item: (String::from(""), ItemType::FullStackable),
            ammount: 0,
        }
    }
}

pub enum ItemType {
    FullStackable,
    FourthStackable,
    OneStacbable,
}

/// Ulits functions for some block entities
/// **You still have to make the block entities, these can just be used to help with the implementation**
pub mod utils {
    use super::{BlockEntity, ItemType};

    pub const MAX_MID_CONTAINER: i8 = 27;

    pub fn calc_fullness(kind: &ItemType, ammount: i32) -> i32 {
        match kind {
            &super::ItemType::FullStackable => ammount / 64,
            &super::ItemType::FourthStackable => ammount / 16,
            &super::ItemType::OneStacbable => ammount / 1,
        }
    }

    /// Calculates the signal strength from a given block entity
    /// Make sure that all slots are "registered"
    /// You can do this by making a slot with `None` as the slot
    ///
    /// There are some exeptions that will have their own functions
    /// See https://minecraft.fandom.com/wiki/Redstone_Comparator#Miscellaneous
    pub fn calc_signal_strength<T: BlockEntity>(block: &T) -> Option<i8> {
        let slots = block.get_all();
        let max_slots = slots.len().checked_sub(1)? as f32;

        // Calculate 'fullness'
        // fullness is the just present items/max items
        let fullness: f32 = {
            let mut count = 0;
            for (_, slot) in slots {
                let slot = slot.unwrap_or_default();
                count += calc_fullness(&slot.item.1, slot.ammount);
            }
            count as f32
        };
        // Some wierd math from minecraft!
        let sum = (1_f32 + (fullness / max_slots) * 14_f32).floor() as i8;
        Some(sum)
    }
}
