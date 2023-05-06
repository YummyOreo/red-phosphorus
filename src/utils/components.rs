use crate::types::PowerLevel;

/// Input # of entities. If none, then just supplie 0
pub fn calc_gold_plate(entities: i16) -> PowerLevel {
    entities.clamp(0, 15) as PowerLevel
}

#[allow(clippy::cast_possible_truncation)]
/// Input # of entities. If none, then just supplie 0
pub fn calc_iron_plate(entities: i16) -> PowerLevel {
    // Each "range" is (powerlevel) - 10 + 1 .. (powerlevel * 10)

    let remainder = entities % 10;
    // Checks if divisable by 10
    if remainder == 0 {
        // If it is, then we can just return what was in the 10's place
        return (entities / 10).clamp(0, 15) as PowerLevel;
    }

    // if not divisable by 10
    // then add 10 to it           ie. 11 + 10 = 21
    // then remove the one's place ie. 21 - 1 = 20
    // then divide by 10           ie. 20 / 10 = 2
    // the product is then clamped to 15, then converted to a i8
    (((entities + 10) - (remainder)) / 10).clamp(0, 15) as PowerLevel
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
