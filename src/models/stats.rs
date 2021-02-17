use std::fmt;

use serde::{Deserialize, Serialize};
use variant_count::VariantCount;
use enum_index_derive::{IndexEnum, EnumIndex};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, VariantCount, EnumIndex, IndexEnum)]
pub enum StatType {
    Strength,
    Precision,
    Endurance,
    Luck,
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct StatBounds {
    pub from: i64,
    pub to: i64,
    pub stat_type: StatType,
}

impl Default for StatBounds {
    fn default() -> Self {
        StatBounds {
            from: 0,
            to: 10,
            stat_type: StatType::Strength,
        }
    }
}
