#![allow(clippy::cast_precision_loss)]
#[allow(clippy::module_name_repetitions)]
pub trait BlockEntity {
    fn get_signal_strength(&self) -> i8;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub item: ItemType,
    pub ammount: i32,
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            item: ItemType::FullStackable,
            ammount: 0,
        }
    }
}

impl Slot {
    pub fn new(kind: ItemType, ammount: i32) -> Self {
        Self {
            item: kind,
            ammount,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemType {
    FullStackable,
    FourthStackable,
    SingleStacbable,
}

/// Ulits functions for some block entities
/// **You still have to make the block entities, these can just be used to help with the implementation**
pub mod utils {
    use super::{ItemType, Slot};

    pub const MAX_MID_CONTAINER: i8 = 27;

    pub const ITEM_SLOT_MAX: f32 = 64_f32;

    /// Converts any slot type to full stack.
    /// Ie. (Fouth Stackable) 16 -> 64
    pub fn get_fullstack_equiv(kind: &ItemType, ammount: f32) -> f32 {
        match *kind {
            ItemType::FullStackable => ammount,
            ItemType::FourthStackable => ammount * 16_f32,
            ItemType::SingleStacbable => ammount * 64_f32,
        }
    }

    fn get_fullness(slots: Vec<Option<Slot>>) -> f32 {
        // Calculate 'fullness'
        // fullness is the just full stack equiv / max items per slot
        let mut fullness = 0_f32;
        for slot in slots {
            let slot = slot.unwrap_or_default();
            fullness += get_fullstack_equiv(&slot.item, slot.ammount as f32);
            println!("{fullness}");
        }
        fullness / ITEM_SLOT_MAX
    }

    fn calc_strength(fullness: f32, max_slots: f32) -> i8 {
        // Some wierd math from minecraft!
        // casting to i8 should work bc strengths should never go over 15
        #[allow(clippy::cast_possible_truncation)]
        {
            let sum = (fullness / max_slots).mul_add(14_f32, 1_f32).floor();
            if sum > 15_f32 {
                15_i8
            } else {
                sum as i8
            }
        }
    }

    /// Calculates the signal strength from a given block entity
    /// Make sure that all slots are "registered"
    /// You can do this by making a slot with `None` as the slot
    ///
    /// There are some exeptions that will have their own functions
    /// See [the wiki](https://minecraft.fandom.com/wiki/Redstone_Comparator#Miscellaneous)
    pub fn calc_signal_strength(slots: Vec<Option<Slot>>) -> Option<i8> {
        let max_slots: f32 = slots.len().checked_sub(1)? as f32;

        let fullness: f32 = get_fullness(slots);
        Some(calc_strength(fullness, max_slots))
    }

    pub fn calc_strength_cake(slices: i8) -> i8 {
        slices * 2_i8
    }

    pub fn calc_strength_jukebox(disk_name: Option<&str>) -> Option<i8> {
        if let Some(name) = disk_name {
            return match name.to_lowercase().as_str() {
                "13" => Some(1),
                "cat" => Some(2),
                "blocks" => Some(3),
                "chirp" => Some(4),
                "far" => Some(5),
                "mall" => Some(6),
                "mellohi" => Some(7),
                "stal" => Some(8),
                "strad" => Some(9),
                "ward" => Some(10),
                "11" => Some(11),
                "wait" => Some(12),
                "pigstep" => Some(13),
                "5" => Some(15),
                _ => None,
            };
        }
        Some(0)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn calc_strength_lectern(pages: i32, current_page: i32) -> i8 {
        ((current_page - 1) as f32 / (pages - 1) as f32)
            .mul_add(14_f32, 1_f32)
            .floor() as i8
    }

    #[cfg(test)]
    mod test {
        use super::get_fullness;
        use crate::block::entity::{utils::calc_strength, ItemType, Slot};

        #[test]
        fn test_fullness() {
            let slots = vec![
                Some(Slot::new(ItemType::FullStackable, 64)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::FourthStackable, 0)),
            ];
            let expected = 128_f32 / 64_f32;

            let res = get_fullness(slots);
            assert_eq!(res, expected);
        }
        #[test]
        fn test_strength() {
            // Emulating hopper w/ strength 4
            let slots = vec![
                Some(Slot::new(ItemType::FullStackable, 5)),
                Some(Slot::new(ItemType::FullStackable, 0)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::FourthStackable, 0)),
                Some(Slot::new(ItemType::FourthStackable, 0)),
            ];
            let expected = 4_i8;

            let res = calc_strength(get_fullness(slots), 5_f32);
            assert_eq!(res, expected);

            // Emulating dispenser w/ strength 15
            let slots = vec![
                Some(Slot::new(ItemType::FullStackable, 64)),
                Some(Slot::new(ItemType::FullStackable, 64)),
                Some(Slot::new(ItemType::FullStackable, 64)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::SingleStacbable, 1)),
                Some(Slot::new(ItemType::FourthStackable, 16)),
                Some(Slot::new(ItemType::FourthStackable, 16)),
            ];
            let expected = 15_i8;

            println!("{}", get_fullness(slots.clone()));

            let res = calc_strength(get_fullness(slots), 9_f32);
            assert_eq!(res, expected);
        }
    }
}
