mod box_trait;

use crate::box_trait::TrapBox;
use crate::box_trait::TreasureBox;
use crate::box_trait::open_box;

fn main() {
    let box1: TreasureBox = TreasureBox::new(128, "20 Gold".to_string());
    let box2: TrapBox = TrapBox::new(-42, "Spider".to_string());

    open_box(&box1, 127);
    open_box(&box1, 128);
    open_box(&box2, -42);
}