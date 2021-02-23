use crate::{app::App, keys, style::SharedTheme, ui::widgets::DrawableComponent};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub struct DiscList {
    pub selection: usize,
    pub focus: bool,
    theme: SharedTheme,
}

impl DiscList {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selection: 0,
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for DiscList {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<()> {
        let title = format!("Discs [{}]", keys::get_hint(app.key_config.disc_list));

        let table = Table::new(vec![
            Row::new(vec!["Name", "Type", "Manufacturer"]),
            Row::new(vec!["Name", "Type", "Manufacturer"]),
            Row::new(vec!["Name", "Type", "Manufacturer"]),
            Row::new(vec!["Name", "Type", "Manufacturer"]),
        ])
        .header(Row::new(vec!["1", "2", "3"]))
        .block(
            Block::default()
                .style(self.theme.block_style(self.focus))
                .borders(Borders::ALL)
                .title(title),
        )
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .column_spacing(1);

        f.render_widget(table, rect);
        Ok(())
    }
}
