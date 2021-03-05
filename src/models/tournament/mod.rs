use itertools::Itertools;

use crate::models::player::Player;

pub struct Tournament {
    player: Vec<Player>,
    stages: Vec<Stage>,
}

impl Tournament {
    pub fn new(player: Vec<Player>) -> Self {
        // first stage
        let first_stage = Stage::new(StageType::FirstStage, player);
        // second stage
        let second_stage = Stage::default().stage(StageType::SecondStage);
        // third stage
        todo!();
    }
}

pub struct Stage {
    brackets: Option<Vec<Bracket>>,
    stage_type: StageType,
}

impl Stage {
    pub fn new(stage_type: StageType, player: Vec<Player>) -> Self {
        let mut brackets: Vec<Bracket> = Vec::new();
        for chunk in &player.into_iter().chunks(4) {
            brackets.push(Bracket::new(chunk.collect()));
        }

        Self {
            stage_type,
            brackets: Some(brackets),
        }
    }

    pub fn stage(mut self, stage_type: StageType) -> Self {
        self.stage_type = stage_type;
        return self;
    }

    pub fn brackets(mut self, brackets: Vec<Bracket>) -> Self {
        self.brackets = Some(brackets);
        return self;
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            stage_type: StageType::FirstStage,
            brackets: None,
        }
    }
}

pub enum StageType {
    FirstStage,
    SecondStage,
}
pub struct Bracket {
    player: Vec<Player>,
}

impl Bracket {
    pub fn new(player: Vec<Player>) -> Self {
        Self { player }
    }
}
