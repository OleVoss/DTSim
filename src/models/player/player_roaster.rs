use serde::{Deserialize, Serialize};

use super::stats::{PlayerStatType, Stat};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRoaster {
    pub player_list: Vec<Player>,
}

impl PlayerRoaster {
    pub fn new() -> Self {
        Self {
            player_list: Vec::new(),
        }
    }

    pub fn by_index(&self, index: usize) -> Option<&Player> {
        self.player_list.get(index)
    }

    pub fn by_index_mut(&mut self, index: usize) -> Option<&mut Player> {
        self.player_list.get_mut(index)
    }

    pub fn player_count(&self) -> usize {
        self.player_list.len()
    }

    pub fn get_player(&self, count: usize) -> Option<Vec<Player>> {
        Some(
            self.player_list
                .iter()
                .take(count)
                .map(|p| p.clone())
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub stats: Option<Vec<Stat>>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            stats: None,
        }
    }
}

impl Player {
    pub fn stat_value(&self, stat_type: PlayerStatType) -> i64 {
        match &self.stats {
            Some(stats) => match stats.iter().find(|s| s.stat_type() == stat_type) {
                Some(stat) => stat.value(),
                None => 0,
            },
            None => 0,
        }
    }

    pub fn change_stat_value(&mut self, new_value: i64, stat_type: PlayerStatType) {
        match &mut self.stats {
            Some(stats) => match stats.iter_mut().find(|s| s.stat_type() == stat_type) {
                Some(stat) => {
                    stat.set_value(new_value);
                }
                None => {}
            },
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_stat_value() {
        let mut player = Player {
            name: "test player".to_string(),
            stats: Some(vec![Stat::new(PlayerStatType::Strength, 5)]),
        };

        player.change_stat_value(7, PlayerStatType::Strength);
        assert!(player.stat_value(PlayerStatType::Strength) == 7);
    }
}
