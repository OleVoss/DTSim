use std::{cmp::Reverse, default, rc::Rc, todo};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use serde::__private::de;
use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, List, ListItem, Tabs}};
use anyhow::{Result, bail};

use crate::{keys::KeyConfig, models::PlayerRoaster};


pub struct App {
    pub should_quit: bool,
    pub tab: usize,
    pub key_config: Rc<KeyConfig>,
    pub player_roaster: PlayerRoaster,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            tab: 3,
            key_config: Rc::new(KeyConfig::init()),
            player_roaster: PlayerRoaster::new(),
        }
    }

    pub fn switch_tab_to(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }



    pub fn key_event(&mut self, ev: KeyEvent) -> Result<()> {
        if ev == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            self.should_quit = true;
            return Ok(())
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
  
}

// private impls
impl App {
    pub fn switch_tab(&mut self, k: KeyEvent) -> Result<()> {
        if k == self.key_config.tab_overview {
            self.set_tab(0)?;
        } else if k == self.key_config.tab_simulation {
            self.set_tab(1)?;
        } else if k == self.key_config.tab_config {
            self.set_tab(2)?;
        } else if k == self.key_config.tab_player {
            self.set_tab(3)?;
        } else if k == self. key_config.tab_discs {        
            self.set_tab(4)?; 
        }

        Ok(())
    }

    fn set_tab(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }
}