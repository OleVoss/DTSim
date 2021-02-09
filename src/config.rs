use std::rc::Rc;

use anyhow::Result;
use serde::__private::de;
use stats::StatBounds;

use crate::models::{stats, StatType};

pub type SharedConfig = Rc<Config>;

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
                    to: 100,
                    stat_type: StatType::Strength,
                },
                StatBounds {
                    from: 0,
                    to: 100,
                    stat_type: StatType::Precision,
                },
                StatBounds {
                    from: 0,
                    to: 100,
                    stat_type: StatType::Endurance,
                },
                StatBounds {
                    from: 0,
                    to: 100,
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
}
