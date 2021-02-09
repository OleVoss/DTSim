use anyhow::Result;

use super::stats::Stat;

#[derive(Debug)]
pub struct PlayerRoaster<'r> {
    pub player_list: Vec<Player<'r>>,
}

impl<'r> PlayerRoaster<'r> {
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

#[derive(Debug, Clone)]
pub struct Player<'p> {
    pub name: &'static str,
    pub stats: Option<Vec<Stat<'p>>>,
}
