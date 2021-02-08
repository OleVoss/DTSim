pub enum StatType {
    Strength { from: i32, to: i32 },
    Precision { from: i32, to: i32 },
    Endurance { from: i32, to: i32 },
    Luck { from: i32, to: i32 },
}

pub struct Stat {
    stat_type: &StatType,
    value: i32,
}
