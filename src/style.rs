use std::rc::Rc;

use tui::style::{Color, Style};

pub type SharedTheme = Rc<Theme>;

pub struct Theme {
    selected_tab: Color,
    selected_fg: Color,
    selected_item_fg: Color,
    disabled_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_tab: Color::Gray,
            selected_fg: Color::Gray,
            selected_item_fg: Color::Red,
            disabled_fg: Color::DarkGray,
        }
    }
}

impl Theme {
    pub fn init() -> Self {
        Theme::default()
    }
    pub fn block(&self, focus: bool) -> Style {
        if focus {
            Style::default()
        } else {
            Style::default().fg(self.disabled_fg)
        }
    }

    pub fn item(&self, selected: bool) -> Style {
        if selected {
            Style::default().fg(self.selected_item_fg)
        } else {
            Style::default()
        }
    }
}
