use std::{convert::TryInto, rc::Rc};

use anyhow::Result;
use serde::__private::de;

use crate::models::player::stats::{StatBounds, PlayerStatType};

pub type SharedConfig = Rc<Config>;

pub const STEPS: i64 = 10;
pub const AVAILABLE_PLAYER_STATS: [PlayerStatType; 4] = [PlayerStatType::Strength, PlayerStatType::Precision, PlayerStatType::Endurance, PlayerStatType::Luck];

#[derive(Debug)]
pub struct Config {
    // stat bounds
    stat_bounds: Vec<StatBounds>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stat_bounds: vec![
                StatBounds::new(
                    0,
                    10,
                    PlayerStatType::Strength
                ),
                StatBounds::new(
                    0,
                    10,
                    PlayerStatType::Endurance
                ),
                StatBounds::new(
                    0,
                    10,
                    PlayerStatType::Precision
                ),
                StatBounds::new(
                    0,
                    10,
                    PlayerStatType::Luck
                ),
            ],
        }
    }
}

impl Config {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn get_bounds(&self, stat_type: PlayerStatType) -> Option<&StatBounds> {
        self.stat_bounds.iter().find(|sb| sb.stat_type() == stat_type)
    }

    pub fn stat_increment_step(&self, stat_type: PlayerStatType) -> i64 {
        match self.get_bounds(stat_type) {
            Some(bounds) => {
                let lower = bounds.from();
                let upper = bounds.to();
                let step: i64 = ((upper - lower) / STEPS).try_into().unwrap();
                step
            }
            None => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::models::player::stats::{PlayerStatType, StatBounds};
    #[test]
    fn stat_increment_step() {
        let config = Config {
            stat_bounds: vec![
                StatBounds::new(0, 10, PlayerStatType::Strength)
            ],
        };
        let step = config.stat_increment_step(PlayerStatType::Strength);
        assert!(step == 1);
    }
}