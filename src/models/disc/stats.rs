use std::fmt;

use enum_index_derive::{EnumIndex, IndexEnum};
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DiscStat {
    stat_type: DiscStatType,
    value: f64,
}

impl DiscStat {
    pub fn new(stat_type: DiscStatType, value: f64) -> Self {
        Self { stat_type, value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn stat_type(&self) -> DiscStatType {
        self.stat_type
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiscStatBounds {
    stat_type: DiscStatType,
    from: f64,
    to: f64,
}

impl Default for DiscStatBounds {
    fn default() -> Self {
        Self {
            stat_type: DiscStatType::Speed,
            from: 0.0,
            to: 13.0,
        }
    }
}

impl DiscStatBounds {
    pub fn new(from: f64, to: f64, stat_type: DiscStatType) -> Self {
        Self {
            stat_type,
            from,
            to,
        }
    }
    pub fn from(&self) -> f64 {
        self.from
    }

    pub fn to(&self) -> f64 {
        self.to
    }

    pub fn stat_type(&self) -> DiscStatType {
        self.stat_type
    }
}

#[derive(
    Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, VariantCount, EnumIndex, IndexEnum,
)]
pub enum DiscStatType {
    Speed,
    Glide,
    Turn,
    Fade,
}

impl fmt::Display for DiscStatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
