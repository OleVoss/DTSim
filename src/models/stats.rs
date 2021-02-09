use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Strength,
    Precision,
    Endurance,
    Luck,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: i32,
}

#[derive(Debug)]
pub struct StatBounds {
    pub from: i64,
    pub to: i64,
    pub stat_type: StatType,
}
