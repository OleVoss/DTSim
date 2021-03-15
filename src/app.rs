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
        };
        if initialize {
            app.load_player();
            app.load_discs();
        }

        return app;
    }
}

// private impls
impl App {
    // TODO: Error handling
    pub fn load_player(&mut self) {
        let assets_path = Path::new("./assets");
        let contents = fs::read_to_string(assets_path.join("player.ron"));
        let roaster: PlayerRoaster = ron::from_str(&contents.unwrap().to_owned()).unwrap();
        self.player_roaster = roaster;
    }

    pub fn load_discs(&mut self) {
        let asset_path = Path::new("./assets");
        let contents = fs::read_to_string(asset_path.join("discs.ron"));
        let storage: DiscStorage = ron::from_str(&contents.unwrap().to_owned()).unwrap();
        self.disc_storage = storage;
    }
}
