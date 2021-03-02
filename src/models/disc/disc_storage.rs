use std::fmt;

use super::stats::{DiscStat, DiscStatType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscStorage {
    pub discs: Vec<Disc>,
}

impl DiscStorage {
    pub fn new() -> Self {
        Self { discs: Vec::new() }
    }

    pub fn by_index(&self, index: usize) -> Option<&Disc> {
        self.discs.get(index)
    }

    pub fn by_index_mut(&mut self, index: usize) -> Option<&mut Disc> {
        self.discs.get_mut(index)
    }

    pub fn disc_count(&self) -> usize {
        self.discs.len() as usize
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disc {
    pub name: String,
    pub manufacturer: String,
    pub disc_type: DiscType,
    pub disc_stats: Vec<DiscStat>,
}

impl Default for Disc {
    fn default() -> Self {
        Self {
            name: String::from("Saint"),
            manufacturer: String::from("Latitude64"),
            disc_type: DiscType::Driver,
            disc_stats: vec![
                DiscStat::new(DiscStatType::Speed, 9.0),
                DiscStat::new(DiscStatType::Glide, 7.0),
                DiscStat::new(DiscStatType::Turn, -1.0),
                DiscStat::new(DiscStatType::Fade, 2.0),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DiscType {
    Driver,
    Midrange,
    Putter,
}

impl fmt::Display for DiscType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
