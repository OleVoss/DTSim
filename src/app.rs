use std::rc::Rc;

use anyhow::{bail, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
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
    models::{Player, PlayerRoaster, Stat, StatType},
};

pub struct App<'a> {
    pub should_quit: bool,
    pub tab: usize,
    pub config: SharedConfig,
    pub key_config: SharedKeyConfig,
    pub player_roaster: PlayerRoaster<'a>,
}

impl<'a> App<'a> {
    pub fn new(initialize: bool) -> App<'a> {
        let app = App {
            should_quit: false,
            tab: 3,
            config: Rc::new(Config::init()),
            key_config: Rc::new(KeyConfig::init()),
            player_roaster: PlayerRoaster::new(),
        };
        if initialize {
            todo!();
        }
        return app;
    }

    pub fn switch_tab_to(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }

    pub fn key_event(&mut self, ev: KeyEvent) -> Result<()> {
        if ev == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            self.should_quit = true;
            return Ok(());
        }

        if ev == self.key_config.tab_overview
            || ev == self.key_config.tab_config
            || ev == self.key_config.tab_simulation
            || ev == self.key_config.tab_player
            || ev == self.key_config.tab_discs
        {
            self.switch_tab(ev)?;
        }

        Ok(())
    }

    pub fn switch_tab(&mut self, k: KeyEvent) -> Result<()> {
        if k == self.key_config.tab_overview {
            self.set_tab(0)?;
        } else if k == self.key_config.tab_simulation {
            self.set_tab(1)?;
        } else if k == self.key_config.tab_config {
            self.set_tab(2)?;
        } else if k == self.key_config.tab_player {
            self.set_tab(3)?;
        } else if k == self.key_config.tab_discs {
            self.set_tab(4)?;
        }

        Ok(())
    }
}

// private impls
impl<'a> App<'a> {
    fn set_tab(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }

    // TODO: Outsource to .ron files or such...
}
