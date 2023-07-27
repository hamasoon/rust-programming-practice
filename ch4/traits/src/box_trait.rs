pub trait Box {
    fn check(&self, key: i32) -> bool;
    fn open(&self);
}

pub fn open_box(bx: &impl Box, key: i32) {
    if bx.check(key) {
        bx.open();
    }
    else {
        println!("Wrong key!")
    }
}

pub struct TreasureBox {
    key: i32,
    treasure: String,
}

impl TreasureBox {
    pub fn new(key: i32, treasure: String) -> TreasureBox{
        TreasureBox { key, treasure }
    }
}

impl Box for TreasureBox {
    fn check(&self, key: i32) -> bool {
        self.key == key
    }

    fn open(&self) {
        println!("Treasure Box! : There is {} inside box", self.treasure);
    }
}

pub struct TrapBox {
    key: i32,
    treasure: String,
}

impl TrapBox {
    pub fn new(key: i32, treasure: String) -> TrapBox{
        TrapBox { key, treasure }
    }
}


impl Box for TrapBox {
    fn check(&self, key: i32) -> bool {
        self.key == key
    }

    fn open(&self) {
        println!("Trap Box! : There is {} inside box", self.treasure);
    }
}