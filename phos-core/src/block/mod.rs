pub mod entity;
pub mod redstone;

use std::mem::swap;

use self::redstone::Component;
use crate::block::entity::BlockEntity;

#[allow(dead_code)]
pub struct Block<'a> {
    name: String,
    position: (i32, i32),
    rotation: Facing,

    is_solid: bool,
    power: i8,
    power_source: Option<&'a Block<'a>>,
    redstone: Option<Component>,

    blockentity: Option<Box<dyn BlockEntity>>,

    hitbox: Option<String>,
    sticky: bool,
    movable: bool,
}

// Getters and setters
impl<'a> Block<'a> {
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    /// Sets new position, then returns the old position
    pub fn set_position(&mut self, position: (i32, i32)) -> (i32, i32) {
        let mut old_position = position;
        swap(&mut self.position, &mut old_position);
        old_position
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Sets new name, then returns the old name
    pub fn set_name(&mut self, name: String) -> String {
        let mut old_name = name;
        swap(&mut self.name, &mut old_name);
        old_name
    }

    pub fn get_rotation(&self) -> &Facing {
        &self.rotation
    }
    /// Sets new rotation, then returns the old rotation
    pub fn set_rotation(&mut self, rotation: Facing) -> Facing {
        let mut old_rotation = rotation;
        swap(&mut self.rotation, &mut old_rotation);
        old_rotation
    }

    pub fn is_solid(&self) -> bool {
        self.is_solid
    }

    pub fn get_power(&self) -> i8 {
        self.power
    }
    /// Sets new power, then returns the old power
    pub fn set_power(&mut self, power: i8) -> i8 {
        let mut old_power = power;
        swap(&mut self.power, &mut old_power);
        old_power
    }

    pub fn get_power_source(&self) -> Option<&'a Block<'a>> {
        self.power_source
    }
    /// Sets new power, then returns the old power
    pub fn set_power_source(&mut self, power_source: &'a Block<'a>) -> Option<&'a Block<'a>> {
        let old_power = self.power_source.take();
        self.power_source = Some(power_source);
        old_power
    }

    pub fn get_redstone(&self) -> Option<&Component> {
        self.redstone.as_ref()
    }
    pub fn get_redstone_mut(&mut self) -> Option<&mut Component> {
        self.redstone.as_mut()
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_block_entity(&self) -> Option<&Box<dyn BlockEntity>> {
        self.blockentity.as_ref()
    }
    #[allow(clippy::borrowed_box)]
    pub fn get_block_entity_mut(&mut self) -> Option<&mut Box<dyn BlockEntity>> {
        self.blockentity.as_mut()
    }

    pub fn is_sticky(&self) -> bool {
        self.sticky
    }
    pub fn is_movable(&self) -> bool {
        self.movable
    }

    pub fn get_hitbox(&self) -> Option<&String> {
        self.hitbox.as_ref()
    }
}

#[derive(Clone)]
pub enum Facing {
    /// -Z
    North,
    /// +Z
    South,
    /// +X
    East,
    /// -X
    West,
    Up,
    Down,
}
