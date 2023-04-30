pub enum Tree<'a> {
    PowerSource {
        power: i8,
        connections: Vec<&'a mut Tree<'a>>,
    },
    Block {
        power: i8,
        connections: Vec<&'a mut Tree<'a>>,
    },
    Air,

    Dust {
        power: i8,
        connections: Vec<&'a mut Tree<'a>>,
    },
    Repeater {
        power: i8,
        delay: i8,
        locked: bool,
        connections: Vec<&'a mut Tree<'a>>,
    },
}

