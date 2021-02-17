use std::{convert::TryInto, rc::Rc};

use anyhow::Result;
use serde::__private::de;
use stats::{Stat, StatBounds};

use crate::models::{stats, StatType};

pub type SharedConfig = Rc<Config>;

pub const STEPS: i64 = 10;
pub const AVAILABLE_STATS: [StatType; 4] = [StatType::Strength, StatType::Precision, StatType::Endurance, StatType::Luck];

#[derive(Debug)]
pub struct Config {
    // stat bounds
    stat_bounds: Vec<StatBounds>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stat_bounds: vec![
                StatBounds {
                    from: 0,
                    to: 10,
                    stat_type: StatType::Strength,
                },
                StatBounds {
                    from: 0,
                    to: 10,
                    stat_type: StatType::Precision,
                },
                StatBounds {
                    from: 0,
                    to: 10,
                    stat_type: StatType::Endurance,
                },
                StatBounds {
                    from: 0,
                    to: 10,
                    stat_type: StatType::Luck,
                },
            ],
        }
    }
}

impl Config {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn get_bounds(&self, stat_type: StatType) -> Option<&StatBounds> {
        self.stat_bounds.iter().find(|sb| sb.stat_type == stat_type)
    }

    pub fn stat_increment_step(&self, stat_type: StatType) -> i32 {
        match self.get_bounds(stat_type) {
            Some(bounds) => {
                let lower = bounds.from;
                let upper = bounds.to;
                let step: i32 = ((upper - lower) / STEPS).try_into().unwrap();
                step
            }
            None => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::models::stats::{StatType, StatBounds};
    #[test]
    fn stat_increment_step() {
        let config = Config {
            stat_bounds: vec![
                StatBounds {
                    from: 0,
                    to: 10,
                    stat_type: StatType::Strength,
                }
            ],
        };
        let step = config.stat_increment_step(StatType::Strength);
        assert!(step == 1);
    }
}