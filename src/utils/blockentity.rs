//! Ulits functions for some block entities
//! **You still have to make the block entities, these can just be used to help with the implementation**
//!
//! There are some provided functions for calculating strength
//!
//! but there are some exceptions that will have their own functions
//! See [the wiki](https://minecraft.fandom.com/wiki/Redstone_Comparator#Miscellaneous)
//! Some have been providid in the utils, others are too simple to be implemented here.

use crate::types::block::entity::{ItemType, Slot};

/// Double chest
pub const MAX_BIG_CONTAINER: i8 = 54;
/// Chest, shulker box, or barrel
pub const MAX_MID_CONTAINER: i8 = 27;
/// Dropper and dispenser
pub const MAX_SMALL_CONTAINER: i8 = 9;
/// Hopper, brewing stand
pub const MAX_TINY_CONTAINER: i8 = 5;
/// Furnaces
pub const MAX_FURNACE_CONTAINER: i8 = 3;

pub const FULL_STACKABLE_MAX: f32 = 64_f32;
pub const FOURTH_STACKABLE_MAX: f32 = 16_f32;
pub const SINGLE_STACKABLE_MAX: f32 = 1_f32;

/// Converts any slot type to full stack.
/// > This will not check if `ammount` is above 64 (this should be impossible in vanilla mc)
///
/// `FullStackable` * 1
/// `FourthStackable` * 4
/// `SingleStackable` * 64
///
/// # Example:
/// ## `SingleStackable`
/// Should times `ammount` by `64`
/// `(ammount) * 64`
/// ```rust
/// use red_phosphorus::types::block::entity::{ItemType};
/// use red_phosphorus::utils::blockentity::get_fullstack_equiv;
///
/// let item_kind: ItemType = ItemType::SingleStackable;
/// let equiv: f32 = get_fullstack_equiv(&item_kind, 1_f32);
/// assert_eq!(64_f32, equiv);
/// ```
///
/// ## `FourthStackable`
/// Should times the given `ammount` by 4
/// `(ammount) * 4`
/// ```rust
/// use red_phosphorus::types::block::entity::{ItemType};
/// use red_phosphorus::utils::blockentity::get_fullstack_equiv;
///
/// let item_kind: ItemType = ItemType::FourthStackable;
/// let equiv: f32 = get_fullstack_equiv(&item_kind, 10_f32);
/// assert_eq!(40_f32, equiv);
/// ```
///
/// ## `FullStackable`
/// Should times the given `ammount` by 1
/// `(ammount) * 1`
/// ```rust
/// use red_phosphorus::types::block::entity::{ItemType};
/// use red_phosphorus::utils::blockentity::get_fullstack_equiv;
///
/// let item_kind: ItemType = ItemType::FullStackable;
/// let equiv: f32 = get_fullstack_equiv(&item_kind, 1_f32);
/// assert_eq!(1_f32, equiv);
/// ```
pub fn get_fullstack_equiv(kind: &ItemType, ammount: f32) -> f32 {
    match *kind {
        ItemType::FullStackable => ammount,
        ItemType::FourthStackable => ammount * 4_f32,
        ItemType::SingleStackable => ammount * 64_f32,
    }
}

/// Get the "fullness" from a given slot.
/// This is pulled from the wiki...
fn get_fullness(slots: Vec<Option<Slot>>) -> f32 {
    // Calculate 'fullness'
    // fullness is the just full stack equiv / max items per slot
    let mut fullness = 0_f32;
    for slot in slots {
        let slot = slot.unwrap_or_default();
        fullness += get_fullstack_equiv(&slot.item, slot.ammount as f32);
    }
    fullness / FULL_STACKABLE_MAX
}

/// Calculate strength based on given fullness of a container and the max slots (not full slots)
///
/// `1 + ((fullness) / (max_slots)) * 14`
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
/// Everything except the expectations [listed here](https://minecraft.fandom.com/wiki/Redstone_Comparator#Miscellaneous)
pub fn calc_signal_strength(slots: Vec<Option<Slot>>) -> Option<i8> {
    let max_slots: f32 = slots.len().checked_sub(1)? as f32;

    let fullness: f32 = get_fullness(slots);
    Some(calc_strength(fullness, max_slots))
}

/// Calculate the strength output of cake
/// just number of slices left * 2
///
/// `(slices_left) * 2`
pub fn calc_strength_cake(slices_left: i8) -> i8 {
    slices_left * 2_i8
}

/// Calculate the strength output of a juxebox
///
/// based on the current disk's name
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
/// Calculate the strength from a lecturen based on the max pages and current page
///
/// `1 + ((current_page) - 1 / (pages - 1)) * 14`
pub fn calc_strength_lectern(pages: i32, current_page: i32) -> i8 {
    ((current_page - 1) as f32 / (pages - 1) as f32)
        .mul_add(14_f32, 1_f32)
        .floor() as i8
}

#[cfg(test)]
mod test {
    use super::{calc_strength, get_fullness, ItemType, Slot};

    #[test]
    fn test_fullness() {
        let slots = vec![
            Some(Slot::new(ItemType::FullStackable, 64)),
            Some(Slot::new(ItemType::SingleStackable, 1)),
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
            Some(Slot::new(ItemType::SingleStackable, 1)),
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
            Some(Slot::new(ItemType::SingleStackable, 1)),
            Some(Slot::new(ItemType::SingleStackable, 1)),
            Some(Slot::new(ItemType::SingleStackable, 1)),
            Some(Slot::new(ItemType::SingleStackable, 1)),
            Some(Slot::new(ItemType::FourthStackable, 16)),
            Some(Slot::new(ItemType::FourthStackable, 16)),
        ];
        let expected = 15_i8;

        let res = calc_strength(get_fullness(slots), 9_f32);
        assert_eq!(res, expected);
    }
}
