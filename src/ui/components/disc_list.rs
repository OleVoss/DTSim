use crate::{app::App, keys, style::SharedTheme, ui::widgets::DrawableComponent};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, ListState, Row, Table, TableState},
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
        let disc_rows: Vec<Row> = app
            .disc_storage
            .discs
            .iter()
            .map(|disc| {
                Row::new(vec![
                    disc.name.to_string(),
                    disc.disc_type.to_string(),
                    disc.manufacturer.to_string(),
                ])
            })
            .collect();

        let table = Table::new(disc_rows)
            .header(
                Row::new(vec!["Name", "Type", "Manufacturer"]).style(
                    Style::default()
                        // .fg(Color::LightBlue)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            )
            .block(
                Block::default()
                    .style(self.theme.block_style(self.focus))
                    .borders(Borders::ALL)
                    .title(title),
            )
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .highlight_style(Style::default().fg(self.theme.selected_item_fg))
            .column_spacing(1);

        let mut table_state = TableState::default();
        table_state.select(Some(self.selection));

        f.render_stateful_widget(table, rect, &mut table_state);
        Ok(())
    }
}
