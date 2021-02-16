use std::rc::Rc;

use tui::{
    style::{Color, Style},
    widgets::Block,
};

pub type SharedTheme = Rc<Theme>;

pub struct Theme {
    pub selected_tab: Color,
    pub selected_fg: Color,
    pub selected_item_fg: Color,
    pub disabled_fg: Color,
    pub strength_hightlight: Color,
    pub precision_highlight: Color,
    pub endurance_highlight: Color,
    pub luck_highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_tab: Color::Gray,
            selected_fg: Color::Gray,
            selected_item_fg: Color::Red,
            disabled_fg: Color::DarkGray,
            strength_hightlight: Color::Red,
            precision_highlight: Color::Blue,
            endurance_highlight: Color::Green,
            luck_highlight: Color::Yellow,
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


    pub fn slider_block(&self, focus: bool) -> Block {
        todo!();
    }
}
