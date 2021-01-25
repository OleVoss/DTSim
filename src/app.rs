use std::{cmp::Reverse, default, rc::Rc, todo};
use crate::{components::DrawableComponent, draw, keys::{KeyConfig, SharedKeyConfig}, tabs::Overview};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use serde::__private::de;
use tui::{Frame, backend::Backend, layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, List, ListItem, Tabs}};
use anyhow::{Result, bail};



pub struct App {
    pub should_quit: bool,
    tab: usize,
    overview_tab: Overview,
    key_config: SharedKeyConfig,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            tab: 0,
            overview_tab: Overview::new(),
            key_config: Rc::new(KeyConfig::init()),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(fsize);

        self.draw_tabs(f, chunks_main[0]);

        match self.tab {
            0 => self.overview_tab.draw(f, chunks_main[1])?,
            _ => bail!("unknown tab"),
        };

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
            || ev == self. key_config.tab_discs
        {
            self.switch_tab(ev)?;
        }

        Ok(())
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        // TODO: https://github.com/extrawurst/gitui/blob/master/src/app.rs Zeile 641-strings editable with config usw.
        let tabs = [
            Span::raw("Overview [1]"),
            Span::raw("Simulation [2]"),
            Span::raw("Config [3]"),
            Span::raw("Player [4]"),
            Span::raw("Discs [5]"),
        ]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

        f.render_widget(
            Tabs::new(tabs)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default())
            )
            .style(Style::default().add_modifier(Modifier::DIM))
            .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED).remove_modifier(Modifier::DIM))
            .divider("|")
            .select(self.tab),
            r,
        );

    }
}

// private impls
impl App {
    fn switch_tab(&mut self, k: KeyEvent) -> Result<()> {
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