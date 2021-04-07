use anyhow::Result;
use nalgebra::Point2;
use std::collections::HashMap;
use DTSim_courses::components::{course::Course, hole};

use crate::models::{disc::disc_storage::Disc, player::Player};

#[derive(Debug, Copy, Clone)]
pub enum ScoreType {
    Ace,
    Eagle,
    Birdie,
    Par,
    Bogey,
    DoubleBogey,
    TrippleBogey,
    IncrediblyBad,
}
pub struct Simulation {
    player_scores: Vec<Scorecard>,
    course: Course,
    hole: i64,
    // environment config
}

impl Simulation {
    pub fn new(course: Course, player: Vec<Player>) -> Self {
        let cards: Vec<Scorecard> = player.into_iter().map(|p| Scorecard::new(p)).collect();
        Self {
            player_scores: cards,
            course,
            hole: 1,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // get player
        let card = self.get_player_at_turn();

        // get hole
        let hole_nr = self.course.active_hole;

        // calculate throw

        // write to scorecard

        // check done

        Ok(())
    }

    fn get_player_at_turn(&mut self) -> &mut Scorecard {
        todo!()
    }

    pub fn get_player_round_score(&self, player: &Player, round: i64) -> Score {
        let player_card = self
            .player_scores
            .iter()
            .find(|ps| ps.player == *player)
            .unwrap();
        if let Some(score) = player_card.scores.get(&round) {
            return score.clone();
        } else {
            return Score::new();
        }
    }
}

pub struct Scorecard {
    player: Player,
    finished: bool,
    scores: HashMap<i64, Score>,
}

impl Scorecard {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            finished: false,
            scores: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, hole_nr: i64, score: Score) {
        self.scores.insert(hole_nr, score);
    }

    pub fn add_stroke_to_round(&mut self, hole_nr: i64, stroke: Stroke) {
        let value = self.scores.get_mut(&hole_nr);
        if let Some(score) = value {
            score.stroke(stroke);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub strokes: Vec<Stroke>,
    pub score_type: Option<ScoreType>,
}

impl Score {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            score_type: None,
        }
    }

    pub fn stroke(&mut self, stroke: Stroke) {
        self.strokes.push(stroke);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Stroke {
    from: Point2<f64>,
    to: Point2<f64>,
}
