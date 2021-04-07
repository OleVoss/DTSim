use anyhow::Result;
use nalgebra::Point2;
use std::{collections::HashMap, rc::Rc};
use DTSim_courses::components::{course::Course, hole::Hole};

use crate::{
    config::{Config, SharedConfig},
    models::{disc::disc_storage::Disc, player::Player},
};

#[derive(Debug, Copy, Clone)]
pub enum ScoreType {
    Ace,
    DoubleEagle,
    Eagle,
    Birdie,
    Par,
    Bogey,
    DoubleBogey,
    TrippleBogey,
    IncrediblyBad,
}

impl ScoreType {
    pub fn from_strokes(strokes: i64, par: i64) -> Self {
        let score = par - strokes;
        match score {
            -3 => Self::Ace,
            score if score == -2 && par == 3 => Self::Ace,
            score if score == -2 && par > 3 => Self::Eagle,
            -1 => Self::Birdie,
            0 => Self::Par,
            1 => Self::Bogey,
            2 => Self::DoubleBogey,
            3 => Self::TrippleBogey,
            score if score > 3 => Self::IncrediblyBad,
            _ => Self::Par,
        }
    }
}

#[derive(Default)]
pub struct Simulation {
    pub player_scores: Vec<Scorecard>,
    pub course: Option<Course>,
    pub active_hole: i64,
    pub done: bool,
    // environment config
    config: SharedConfig,
}

impl Simulation {
    pub fn new(course: Course, player: Vec<Player>) -> Self {
        let cards: Vec<Scorecard> = player.into_iter().map(|p| Scorecard::new(p)).collect();
        Self {
            player_scores: cards,
            course: Some(course),
            active_hole: 1,
            done: false,
            config: Rc::new(Config::init()),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // check done

        let hole_nr = self.active_hole;

        // get player
        let card = self.get_player_at_turn();

        // check if player has score arr

        // calculate throw
        let (stroke, hit) = Self::calculate_throw(card);

        // write to scorecard
        if hit {
            card.add_stroke_to_round(hole_nr, stroke);
            card.finish_round(hole_nr, 3); // todo: change to actual hole par value
        } else {
            card.add_stroke_to_round(hole_nr, stroke);
        };

        // check for next round

        Ok(())
    }

    fn get_player_at_turn(&self) -> &mut Scorecard {
        todo!()
    }

    fn calculate_throw(card: &mut Scorecard) -> (Stroke, bool) {
        todo!()

        // use calculator for calc

        // build stroke
    }

    pub fn get_player_round_score(&self, player: &Player, round: i64) -> RoundScore {
        let player_card = self
            .player_scores
            .iter()
            .find(|ps| ps.player == *player)
            .unwrap();
        if let Some(score) = player_card.scores.get(&round) {
            return score.clone();
        } else {
            return RoundScore::new();
        }
    }

    pub fn course_length(&self) -> i64 {
        if let Some(c) = &self.course {
            return *c.length();
        } else {
            return 0;
        }
    }

    pub fn get_active_hole(&self) -> Option<&Hole> {
        if let Some(c) = &self.course {
            return c.holes().get((self.active_hole - 1) as usize); // -1 because auf indexing
        } else {
            return None;
        }
    }
}

pub struct Scorecard {
    player: Player,
    finished: bool,
    scores: HashMap<i64, RoundScore>,
}

impl Scorecard {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            finished: false,
            scores: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, hole_nr: i64, score: RoundScore) {
        self.scores.insert(hole_nr, score);
    }

    pub fn add_stroke_to_round(&mut self, hole_nr: i64, stroke: Stroke) {
        let value = self.scores.get_mut(&hole_nr);
        if let Some(score) = value {
            score.stroke(stroke);
        }
    }

    fn finish_round(&mut self, hole_nr: i64, par: i64) {
        let value = self.scores.get_mut(&hole_nr);
        if let Some(score) = value {
            let strokes = score.strokes.len() as i64;
            score.set_type(ScoreType::from_strokes(strokes, par));
            score.points = par - strokes;
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoundScore {
    pub strokes: Vec<Stroke>,
    pub score_type: Option<ScoreType>,
    pub points: i64,
}

impl RoundScore {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            score_type: None,
            points: 0,
        }
    }

    pub fn stroke(&mut self, stroke: Stroke) {
        self.strokes.push(stroke);
    }

    pub fn set_type(&mut self, score_type: ScoreType) {
        self.score_type = Some(score_type);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Stroke {
    from: Point2<f64>,
    to: Point2<f64>,
    // todo: function path
}
