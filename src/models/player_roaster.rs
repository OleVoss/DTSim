use super::stats::Stat;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRoaster {
    pub player_list: Vec<Player>,
}

impl<'r> PlayerRoaster {
    pub fn new() -> Self {
        Self {
            player_list: Vec::new(),
        }
    }

    pub fn by_index(&self, index: usize) -> Option<&Player> {
        self.player_list.get(index)
    }

    pub fn player_count(&self) -> usize {
        self.player_list.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub stats: Option<Vec<Stat>>,
}
