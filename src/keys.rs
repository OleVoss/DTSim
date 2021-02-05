use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub type SharedKeyConfig = Rc<KeyConfig>;

#[derive(Debug)]
pub struct KeyConfig {
    pub tab_overview: KeyEvent,
    pub tab_simulation: KeyEvent,
    pub tab_config: KeyEvent,
    pub tab_player: KeyEvent,
    pub tab_discs: KeyEvent,
    pub select: KeyEvent,
    pub move_up: KeyEvent,
    pub move_down: KeyEvent,
    pub move_left: KeyEvent,
    pub move_right: KeyEvent,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            tab_overview: KeyEvent { code: KeyCode::Char('1'), modifiers: KeyModifiers::empty()},
            tab_simulation: KeyEvent { code: KeyCode::Char('2'), modifiers: KeyModifiers::empty()},
            tab_config: KeyEvent { code: KeyCode::Char('3'), modifiers: KeyModifiers::empty()},
            tab_player: KeyEvent { code: KeyCode::Char('4'), modifiers: KeyModifiers::empty()},
            tab_discs: KeyEvent { code: KeyCode::Char('5'), modifiers: KeyModifiers::empty()},
            select: KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::empty()},
            move_up: KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::empty()},
            move_down: KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::empty()},
            move_left: KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::empty()},
            move_right: KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::empty()},
        }
    }
}

impl KeyConfig {
    pub fn init() -> Self {
        Self::default()
    }
}