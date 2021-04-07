use std::fmt;

use enum_index_derive::{EnumIndex, IndexEnum};
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stat {
    stat_type: PlayerStatType,
    value: i64,
}

impl Stat {
    pub fn new(stat_type: PlayerStatType, value: i64) -> Self {
        Self { stat_type, value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    pub fn stat_type(&self) -> PlayerStatType {
        self.stat_type
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatBounds {
    stat_type: PlayerStatType,
    from: i64,
    to: i64,
}

impl Default for StatBounds {
    fn default() -> Self {
        StatBounds {
            stat_type: PlayerStatType::Strength,
            from: 0,
            to: 10,
        }
    }
}

impl StatBounds {
    pub fn new(from: i64, to: i64, stat_type: PlayerStatType) -> Self {
        Self {
            stat_type,
            from,
            to,
        }
    }
    pub fn from(&self) -> i64 {
        self.from
    }

    pub fn to(&self) -> i64 {
        self.to
    }

    pub fn stat_type(&self) -> PlayerStatType {
        self.stat_type
    }
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, VariantCount, EnumIndex, IndexEnum,
)]
pub enum PlayerStatType {
    Strength,
    Precision,
    Endurance,
    Luck,
}

impl fmt::Display for PlayerStatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
