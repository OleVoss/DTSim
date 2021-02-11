use std::rc::Rc;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    config::SharedConfig,
    keys::{KeyConfig, SharedKeyConfig},
    ui::tabs::PlayerTabSections,
    UI,
};

pub fn key_event(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
        app.should_quit = true;
        return Ok(());
    }

    if ev == app.key_config.tab_overview
        || ev == app.key_config.tab_config
        || ev == app.key_config.tab_simulation
        || ev == app.key_config.tab_player
        || ev == app.key_config.tab_discs
    {
        app.switch_tab(ev)?;
    }

    if ev == app.key_config.select {
        app.load_player();
    }

    match app.tab {
        0 => (),
        1 => (),
        2 => (),
        3 => player_tab(app, ev, ui)?,
        _ => (),
    }

    Ok(())
}

fn player_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == app.key_config.player_list {
        ui.player_tab.select(PlayerTabSections::Player);
    } else if ev == app.key_config.slider_list {
        ui.player_tab.select(PlayerTabSections::Stats);
    } else if ev == app.key_config.disc_bag {
        ui.player_tab.select(PlayerTabSections::Bag);
    }
    Ok(())
}
