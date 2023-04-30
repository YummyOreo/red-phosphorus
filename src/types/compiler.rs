pub struct BasicState<'a> {
    pub power: i8,
    pub connections: Vec<&'a mut Tree<'a>>,
}

pub enum Tree<'a> {
    PowerSource {
        basic_state: BasicState<'a>,
    },
    Block {
        basic_state: BasicState<'a>,
    },
    Air,

    Dust {
        basic_state: BasicState<'a>,
    },
    Repeater {
        delay: i8,
        locked: bool,
        basic_state: BasicState<'a>,
    },
    Lamp {
        basic_state: BasicState<'a>,
    },
}
