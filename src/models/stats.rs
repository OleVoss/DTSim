#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Strength,
    Precision,
    Endurance,
    Luck,
}

#[derive(Debug, Clone, Copy)]
pub struct Stat<'s> {
    stat_type: &'s StatType,
    value: i32,
}

#[derive(Debug)]
pub struct StatBounds {
    pub from: i64,
    pub to: i64,
    pub stat_type: StatType,
}
