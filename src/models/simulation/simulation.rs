use anyhow::Result;
use std::collections::HashMap;
use DTSim_courses::components::course::Course;

use crate::models::{disc::disc_storage::Disc, player::Player};

pub enum ScoreType {
    Ace,
    Eagle,
    Birdie,
    Par,
    Bogey,
    DoubleBogey,
}
pub struct Simulation {
    scorecard: Scorecard,
    course: Course,
    hole: i64,
    // environment config
}

impl Simulation {
    pub fn new(course: Course, player: Vec<Player>) -> Self {
        Self {
            scorecard: Scorecard::new(player),
            course,
            hole: 1,
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // get player

        // get hole
        let hole_nr = self.course.active_hole;

        // execute throw

        // write score

        // check done

        Ok(())
    }
}

pub struct Scorecard {
    player: Vec<Player>,
    finished: bool,
    scores: HashMap<i64, Score>,
}

impl Scorecard {
    pub fn new(player: Vec<Player>) -> Self {
        Self {
            player,
            finished: false,
            scores: HashMap::new(),
        }
    }
}

pub struct Score {
    strokes: i64,
    score_type: ScoreType,
}
