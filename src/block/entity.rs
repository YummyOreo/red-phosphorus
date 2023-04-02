pub trait BlockEntity {
    fn set_slot(&mut self, slot_name: SlotName, slot_content: Slot);
    fn get_slot(&self, slot_name: SlotName) -> Option<Slot>;

    fn get_all(&self) -> Vec<(SlotName, Slot)>;
}

pub enum SlotName {
    Num(i32),
    Name(String),
}

#[allow(dead_code)]
pub struct Slot {
    item: String,
    ammount: i32,
}
