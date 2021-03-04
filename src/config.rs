use std::{convert::TryInto, rc::Rc};

use anyhow::Result;
use serde::__private::de;

use crate::models::{
    disc::stats::{DiscStatBounds, DiscStatType},
    player::stats::{PlayerStatType, StatBounds},
};

pub type SharedConfig = Rc<Config>;

pub const STEPS: i64 = 10;
pub const AVAILABLE_PLAYER_STATS: [PlayerStatType; 4] = [
    PlayerStatType::Strength,
    PlayerStatType::Precision,
    PlayerStatType::Endurance,
    PlayerStatType::Luck,
];
pub const AVAILABLE_DISC_STATS: [DiscStatType; 4] = [
    DiscStatType::Speed,
    DiscStatType::Glide,
    DiscStatType::Turn,
    DiscStatType::Fade,
];

#[derive(Debug)]
pub struct Config {
    // stat bounds
    player_stat_bounds: Vec<StatBounds>,
    disc_stat_bounds: Vec<DiscStatBounds>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_stat_bounds: vec![
                StatBounds::new(0, 10, PlayerStatType::Strength),
                StatBounds::new(0, 10, PlayerStatType::Endurance),
                StatBounds::new(0, 10, PlayerStatType::Precision),
                StatBounds::new(0, 10, PlayerStatType::Luck),
            ],
            disc_stat_bounds: vec![
                DiscStatBounds::new(0.0, 14.0, DiscStatType::Speed),
                DiscStatBounds::new(1.0, 7.0, DiscStatType::Glide),
                DiscStatBounds::new(-5.0, 1.0, DiscStatType::Turn),
                DiscStatBounds::new(0.0, 5.0, DiscStatType::Fade),
            ],
        }
    }
}

impl Config {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn get_player_bounds(&self, stat_type: PlayerStatType) -> Option<&StatBounds> {
        self.player_stat_bounds
            .iter()
            .find(|sb| sb.stat_type() == stat_type)
    }

    pub fn get_disc_bounds(&self, stat_type: DiscStatType) -> Option<&DiscStatBounds> {
        self.disc_stat_bounds
            .iter()
            .find(|dsb| dsb.stat_type() == stat_type)
    }

    pub fn stat_increment_step(&self, stat_type: PlayerStatType) -> i64 {
        match self.get_player_bounds(stat_type) {
            Some(bounds) => {
                let lower = bounds.from();
                let upper = bounds.to();
                let step: i64 = ((upper - lower) / STEPS).try_into().unwrap();
                step
            }
            None => 0,
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
            player_stat_bounds: vec![StatBounds::new(0, 10, PlayerStatType::Strength)],
            disc_stat_bounds: Vec::new(),
        };
        let step = config.stat_increment_step(PlayerStatType::Strength);
        assert!(step == 1);
    }
}
