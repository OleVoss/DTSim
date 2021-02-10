use std::rc::Rc;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::App,
    config::SharedConfig,
    keys::{KeyConfig, SharedKeyConfig},
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
    let key_config = Rc::new(KeyConfig::init());
    if app.player_roaster.player_count() > 0 {
        if ev == KeyEvent::new(KeyCode::Up, KeyModifiers::empty()) {
            let new_index = (ui.player_tab.selected + app.player_roaster.player_count() - 1)
                % app.player_roaster.player_count();
            ui.player_tab.selected = new_index;
        } else if ev == KeyEvent::new(KeyCode::Down, KeyModifiers::empty()) {
            let new_index = (ui.player_tab.selected + app.player_roaster.player_count() + 1)
                % app.player_roaster.player_count();
            ui.player_tab.selected = new_index;
        } else if ev == key_config.move_right {
            ui.player_tab.slider_test_value += 1.0;
        } else if ev == key_config.move_left {
            ui.player_tab.slider_test_value -= 1.0;
        }
    };
    Ok(())
}
