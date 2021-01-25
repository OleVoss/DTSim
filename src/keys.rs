use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::__private::de;

pub type SharedKeyConfig = Rc<KeyConfig>;

#[derive(Debug)]
pub struct KeyConfig {
    pub tab_overview: KeyEvent,
    pub tab_simulation: KeyEvent,
    pub tab_config: KeyEvent,
    pub tab_player: KeyEvent,
    pub tab_discs: KeyEvent,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            tab_overview: KeyEvent { code: KeyCode::Char('1'), modifiers: KeyModifiers::empty()},
            tab_simulation: KeyEvent { code: KeyCode::Char('2'), modifiers: KeyModifiers::empty()},
            tab_config: KeyEvent { code: KeyCode::Char('3'), modifiers: KeyModifiers::empty()},
            tab_player: KeyEvent { code: KeyCode::Char('4'), modifiers: KeyModifiers::empty()},
            tab_discs: KeyEvent { code: KeyCode::Char('5'), modifiers: KeyModifiers::empty()},
        }
    }
}

impl KeyConfig {
    pub fn init() -> Self {
        Self::default()
    }
}