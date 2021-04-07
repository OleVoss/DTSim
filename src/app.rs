use std::{fs, path::Path, rc::Rc};

use anyhow::{bail, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use de::size_hint::from_bounds;
use fs::read_to_string;
use serde::__private::de;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Tabs},
    Frame,
};
use DTSim_courses::components::{
    area::{Area, AreaDirection, AreaType},
    course::Course,
    hole::Hole,
};

use crate::{
    config::{self, Config, SharedConfig},
    draw,
    keys::{KeyConfig, SharedKeyConfig},
    models::{
        disc::disc_storage::{Disc, DiscStorage},
        player::{Player, PlayerRoaster, PlayerStatType, Stat},
        simulation::simulation::Simulation,
    },
};

pub struct App {
    pub simulation: Option<Simulation>,
    pub should_quit: bool,
    pub config: SharedConfig,
    pub key_config: SharedKeyConfig,
    pub player_roaster: PlayerRoaster,
    pub disc_storage: DiscStorage,
    pub course: Course,
}

impl App {
    pub fn new(initialize: bool) -> App {
        let mut app = App {
            simulation: None,
            should_quit: false,
            config: Rc::new(Config::init()),
            key_config: Rc::new(KeyConfig::init()),
            player_roaster: PlayerRoaster::new(),
            disc_storage: DiscStorage::new(),
            course: Course::new("Course".to_string(), 1),
        };

        if initialize {
            app.load_player();
            app.load_discs();
            app.load_course();
        };

        let simulation = Simulation::new(app.course, app.player_roaster.player_list);
        app.simulation = Some(simulation);

        return app;
    }
}

// private impls
impl App {
    // TODO: Error handling
    pub fn load_player(&mut self) {
        let contents = include_str!("../assets/player.ron");
        let roaster: PlayerRoaster = ron::from_str(&contents).unwrap();
        self.player_roaster = roaster;
    }

    pub fn load_discs(&mut self) {
        let contents = include_str!("../assets/discs.ron");
        let storage: DiscStorage = ron::from_str(&contents).unwrap();
        self.disc_storage = storage;
    }

    pub fn load_course(&mut self) {
        let contents = include_str!("../assets/course_1.ron");
        let course: Course = ron::from_str(&contents).unwrap();
        self.course = course;
    }
}
